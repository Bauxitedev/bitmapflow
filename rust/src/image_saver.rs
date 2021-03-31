use std::{fs::File, path::Path};

use anyhow::Result;
use gdnative::{api::ImageTexture, prelude::*};
use gif::{Encoder, Repeat};
use image::{ImageBuffer, Rgba};
use log::*;
use rayon::prelude::*;

use crate::{frame::texture_to_image, utility::do_with_image_holder};

type Base = Node;
//Base refers to the type ImageSaver inherits from. In this case it's Node (because #[inherit(Node)])

#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register_signals)]
pub struct ImageSaver {}

impl ImageSaver {
    fn new(_owner: &Node) -> Self {
        ImageSaver {}
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "image_save_success",
            args: &[SignalArgument {
                name: "filenames",
                default: Variant::from_array(&VariantArray::new_shared()),
                export_info: ExportInfo::new(VariantType::StringArray),
                usage: PropertyUsage::DEFAULT,
            }],
        });
        builder.add_signal(Signal {
            name: "image_save_failure",
            args: &[SignalArgument {
                name: "error",
                default: Variant::from_str("Dummy error"),
                export_info: ExportInfo::new(VariantType::GodotString),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    fn save_gif(&mut self, filename: &str, fps: f64) -> Result<()> {
        let mut image = File::create(filename)?;

        do_with_image_holder(|imageholder, _owner| {
            let frames = &imageholder.output_frames;
            assert!(!frames.is_empty());

            let width = frames[0].width();
            let height = frames[1].height();

            for frame in frames {
                assert!(
                    frame.width() == width,
                    "Frame width doesn't match: {} != {}",
                    frame.width(),
                    width
                );
                assert!(
                    frame.height() == height,
                    "Frame height doesn't match: {} != {}",
                    frame.height(),
                    height
                );
            }

            let delay_msec = 1000.0 / fps.max(0.01);
            let delay_csec = ((delay_msec / 10.0).ceil()).clamp(0.0, u16::MAX as f64) as u16;
            //NOTE - as of 12-3-2021 there is STILL no safe way to convert a float to u16.
            //If it doesn't fit you get UB. So there's that. https://github.com/rust-lang/rust/issues/10184

            let mut encoder = Encoder::new(&mut image, width as u16, height as u16, &[])?;
            encoder.set_repeat(Repeat::Infinite)?;

            info!("Saving gif with frame delay = {}csec...", delay_csec);

            let gif_frames: Vec<_> = frames
                .par_iter()
                .enumerate()
                .map(|(_i, frame)| {
                    let mut raw_pixels = vec![];

                    for pixel in frame.pixels() {
                        raw_pixels.extend(&pixel.0);
                    }

                    Self::alpha_hack(&mut raw_pixels);

                    let mut gif_frame =
                        gif::Frame::from_rgba(width as u16, height as u16, &mut raw_pixels);

                    gif_frame.delay = delay_csec;
                    gif_frame.dispose = gif::DisposalMethod::Background;

                    gif_frame
                })
                .collect();

            for gif_frame in gif_frames {
                encoder.write_frame(&gif_frame)?;
            }

            Ok(())
        })
        .unwrap()
    }

    fn alpha_hack(raw_pixels: &mut [u8]) {
        assert!(raw_pixels.len() % 4 == 0);
        //Little hack: if you don't do this then some frames will randomly have a black background!
        for [r, g, b, a] in raw_pixels.array_chunks_mut::<4>() {
            if *a == 0 {
                *r = 0;
                *g = 0;
                *b = 0;
            }
        }
    }

    fn save_separate_frames(&mut self, base_filename: &str) -> Result<Vec<String>> {
        let base_filename = base_filename.replace(".png", "");

        //TODO detect file extension using regex and add it back in at the end, see
        // https://stackoverflow.com/questions/6582171/javascript-regex-for-matching-extracting-file-extension

        do_with_image_holder(|imageholder, _owner| {
            let frames = &imageholder.output_frames;
            assert!(!frames.is_empty());

            let results: Vec<Result<_>> = frames
                .par_iter()
                .enumerate()
                .map(|(i, img)| {
                    let filename = format!("{}{:04}.png", base_filename, i);
                    img.save(filename.clone())?;

                    Ok(filename)
                })
                .collect();

            let mut filenames = vec![];
            for result in results {
                let filename = result?;
                filenames.push(filename);
            }

            Ok(filenames)
        })
        .unwrap()
    }

    fn save_spritesheet(
        &mut self,
        filename: &str,
        tex: Ref<ImageTexture, Shared>,
    ) -> Result<String> {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = texture_to_image(tex);
        img.save(filename.to_string())?;

        Ok(filename.to_string())
    }
}

#[methods]
impl ImageSaver {
    #[export]
    fn _on_ui_exported_gif(&mut self, owner: &Base, filename: String, fps: f64) {
        match self.save_gif(&filename, fps) {
            Ok(()) => {
                let filename_only = Path::new(&filename).file_name().unwrap().to_str().unwrap();

                info!("Saved gif succesfully: {:?}", filename);

                owner.emit_signal(
                    "image_save_success",
                    &[Variant::from_string_array(&TypedArray::from_vec(vec![
                        filename_only.into(),
                    ]))],
                )
            }
            Err(err) => {
                let err_str = format!("Failed to save gif as {}: {}", filename, err.to_string());
                warn!("{}", err_str);
                owner.emit_signal("image_save_failure", &[Variant::from_str(err_str)])
            }
        };
    }

    #[export]
    fn _on_ui_exported_separate_frames(&mut self, owner: &Base, base_filename: String) {
        match self.save_separate_frames(&base_filename) {
            Ok(filenames) => {
                let strings = &filenames
                    .iter()
                    .map(|s| {
                        let filename_only = Path::new(&s).file_name().unwrap().to_str().unwrap();
                        GodotString::from(filename_only)
                    })
                    .collect();
                info!("Saved separate frames succesfully: {:?}", filenames);
                owner.emit_signal("image_save_success", &[Variant::from_string_array(strings)])
            }
            Err(err) => {
                let err_str = format!(
                    "Failed to save separate frames as {}: {}",
                    base_filename,
                    err.to_string()
                );
                warn!("{}", err_str);

                owner.emit_signal("image_save_failure", &[Variant::from_str(err_str)])
            }
        };
    }

    #[export]
    fn _on_ui_exported_spritesheet(
        &mut self,
        owner: &Base,
        filename: String,
        tex: Ref<ImageTexture, Shared>,
    ) {
        match self.save_spritesheet(&filename, tex) {
            Ok(filename) => {
                let filename_only = Path::new(&filename).file_name().unwrap().to_str().unwrap();

                info!("Saved spritesheet succesfully: {:?}", filename);

                owner.emit_signal(
                    "image_save_success",
                    &[Variant::from_string_array(&TypedArray::from_vec(vec![
                        filename_only.into(),
                    ]))],
                )
            }
            Err(err) => {
                let err_str = format!(
                    "Failed to save spritesheet as {}: {}",
                    filename,
                    err.to_string()
                );
                warn!("{}", err_str);
                owner.emit_signal("image_save_failure", &[Variant::from_str(err_str)])
            }
        };
    }
}
