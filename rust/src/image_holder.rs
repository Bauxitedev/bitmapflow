use std::fs::File;

use anyhow::{Error, Result};
use gdnative::prelude::*;
use image::{io::Reader as ImageReader, RgbaImage};
use log::*;
use rayon::prelude::*;

use crate::frame::{Frame, Frames};

type Base = Node;
//Base refers to the type ImageHolder inherits from. In this case it's Node (because #[inherit(Node)])

#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register_signals)]
pub struct ImageHolder {
    input_frames: Frames,
    pub output_frames: Frames,
}

impl ImageHolder {
    fn new(_owner: &Node) -> Self {
        ImageHolder {
            input_frames: vec![],
            output_frames: vec![],
        }
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "image_loaded",
            args: &[SignalArgument {
                name: "frames",
                default: Variant::from_array(&VariantArray::new_shared()),
                export_info: ExportInfo::new(VariantType::VariantArray),
                usage: PropertyUsage::DEFAULT,
            }],
        });
    }

    fn update_input_frames(&mut self, owner: TRef<'_, Base>, frames: Frames) {
        self.input_frames = frames;
        owner.emit_signal("image_loaded", &[self.input_frames.to_variant()]);
    }

    fn load_gif(&mut self, filename: &String) -> Result<Frames, Error> {
        let file = File::open(filename)?;
        let mut gif_opts = gif::DecodeOptions::new();
        gif_opts.set_color_output(gif::ColorOutput::Indexed);

        let mut decoder = gif_opts.read_info(file)?;
        let mut screen = gif_dispose::Screen::new_decoder(&decoder);

        let mut new_input_frames = vec![];

        while let Some(frame) = decoder.read_next_frame()? {
            screen.blit_frame(&frame)?;
            let pixels: Frame = Frame::from(screen.pixels.clone());
            new_input_frames.push(pixels);
        }

        Ok(new_input_frames)
    }

    fn load_separate_frames(&mut self, filenames: &StringArray) -> Result<Frames, Error> {
        let images: Vec<Result<RgbaImage, Error>> = filenames
            .read()
            .par_iter()
            .map(|filename| {
                let img = ImageReader::open(filename.to_string())?.decode()?;
                Ok(img.to_rgba8())
            })
            .collect();

        let mut new_input_frames = vec![];
        for img in images {
            let img: RgbaImage = img?;
            let converted_img = Frame(img);
            new_input_frames.push(converted_img);
        }

        Ok(new_input_frames)
    }

    fn load_spritesheet(&mut self, filename: &str, rects: &[Rect2]) -> Result<Frames, Error> {
        let img = ImageReader::open(filename.to_string())?.decode()?;

        //NOTE: crop_imm may not actually do the cropping so this parallellization may not improve perf
        let images: Vec<_> = rects
            .par_iter()
            .map(|rect| {
                img.crop_imm(
                    rect.origin.x.round() as u32,
                    rect.origin.y.round() as u32,
                    rect.width().round() as u32,
                    rect.height().round() as u32,
                )
            })
            .collect();

        let mut new_input_frames = vec![];
        for img in images {
            let img: RgbaImage = img.to_rgba8();
            let converted_img = Frame(img);
            new_input_frames.push(converted_img);
        }

        Ok(new_input_frames)
    }
}

#[methods]
impl ImageHolder {
    #[export]
    fn has_output_frames(&mut self, _owner: &Base) -> bool {
        !self.output_frames.is_empty()
    }

    #[export]
    fn has_input_frames(&mut self, _owner: &Base) -> bool {
        !self.input_frames.is_empty()
    }

    #[export]
    fn get_speed_ratio(&mut self, _owner: &Base) -> Option<f64> {
        if self.has_input_frames(_owner) && self.has_output_frames(_owner) {
            return Some(self.output_frames.len() as f64 / self.input_frames.len() as f64);
        }

        None
    }

    #[export]
    fn _on_imageprocessor_image_processed(&mut self, _owner: &Base, frames: Frames) {
        self.output_frames = frames;
    }

    #[export]
    fn _on_ui_loaded_gif(&mut self, owner: TRef<'_, Base>, filename: String) {
        match self.load_gif(&filename) {
            Ok(frames) => {
                self.output_frames.clear();
                self.update_input_frames(owner, frames);
            }
            Err(err) => {
                error!("Failed to load gif called {}: {:?}", filename, err);
            }
        }
    }

    #[export]
    fn _on_ui_loaded_separate_frames(&mut self, owner: TRef<'_, Base>, filenames: StringArray) {
        match self.load_separate_frames(&filenames) {
            Ok(frames) => {
                self.output_frames.clear();
                self.update_input_frames(owner, frames);
            }
            Err(err) => {
                error!(
                    "Failed to load separate frames called {:?}: {:?}",
                    filenames, err
                );
            }
        }
    }

    #[export]
    fn _on_ui_loaded_spritesheet(
        &mut self,
        owner: TRef<'_, Base>,
        filename: String,
        rects: Vec<Rect2>,
    ) {
        match self.load_spritesheet(&filename, &rects) {
            Ok(frames) => {
                self.output_frames.clear();
                self.update_input_frames(owner, frames);
            }
            Err(err) => {
                error!("Failed to load spritesheet called {}: {:?}", filename, err);
            }
        }
    }
}
