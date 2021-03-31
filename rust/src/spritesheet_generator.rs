use std::time::Instant;

use gdnative::{api::ImageTexture, prelude::*};
use image::{imageops::replace, ImageBuffer, RgbaImage};
use log::*;

use crate::{datatypes::SpritesheetGenerationParams, utility::do_with_image_holder};

type Base = Node;
//Base refers to the type SpritesheetGenerator inherits from. In this case it's Node (because #[inherit(Node)])

#[derive(NativeClass)]
#[inherit(Base)]
pub struct SpritesheetGenerator {}

impl SpritesheetGenerator {
    fn new(_owner: &Base) -> Self {
        SpritesheetGenerator {}
    }
}

#[methods]
impl SpritesheetGenerator {
    #[export]
    fn generate_spritesheet(
        &mut self,
        _owner: &Base,
        spritesheet_params: SpritesheetGenerationParams,
    ) -> Ref<ImageTexture, Unique> {
        do_with_image_holder(|imageholder, _owner| {
            let frames = &imageholder.output_frames;
            let frames_per_row = spritesheet_params.frames_per_row.min(frames.len());

            let frame_width = frames[0].width() as usize;
            let frame_height = frames[0].height() as usize;
            let rows = (frames.len() as f32 / frames_per_row as f32).ceil() as usize;

            let final_img_width = frame_width * frames_per_row;
            let final_img_height = frame_height * rows;

            let mut spritesheet: RgbaImage =
                ImageBuffer::new(final_img_width as u32, final_img_height as u32);

            let start = Instant::now();
            for (i, frame) in frames.iter().enumerate() {
                let x = (i % frames_per_row) * frame_width;
                let y = (i / frames_per_row) * frame_height;
                replace(&mut spritesheet, &(frame.0), x as u32, y as u32);
            }
            let duration = start.elapsed();

            info!("Time elapsed in replacement is: {:?}", duration);

            let data: Vec<u8> = spritesheet
                .pixels()
                .flat_map(|rgba| rgba.0.iter().cloned())
                .collect();

            let texture = ImageTexture::new();
            let image = Image::new();

            image.create_from_data(
                final_img_width as i64,
                final_img_height as i64,
                false,
                Image::FORMAT_RGBA8,
                TypedArray::from_vec(data),
            );

            texture.create_from_image(image, 0);

            texture
        })
        .unwrap()
    }
}
