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
    fn get_optimal_spritesheet_params(&mut self, _owner: &Base) -> SpritesheetGenerationParams {
        // Maximize spritesheet squareness and try to get close to a power of 2

        // This is not the fastest algorithm,
        // but since the spritesheet generation time is dominated by generate_spritesheet(),
        // it doesn't really matter

        let frame_count = do_with_image_holder(|imageholder, _owner| {
            let frames = &imageholder.output_frames;
            frames.len()
        })
        .unwrap();

        let start = Instant::now();

        let closest_to_po2 = (1..)
            .take_while(|frames_per_row| *frames_per_row <= frame_count)
            .map(|frames_per_row| SpritesheetGenerationParams { frames_per_row })
            .min_by_key(|params| {
                let (w, h) = Self::get_spritesheet_size(params);
                let (w, h) = (w as u64, h as u64);

                let (w_nearest_po2, h_nearest_po2) = (w.next_power_of_two(), h.next_power_of_two());

                let (w_po2_diff, h_po2_diff) = (
                    (w_nearest_po2 as i32 - w as i32).abs() as u64,
                    (h_nearest_po2 as i32 - h as i32).abs() as u64,
                );

                let cost = w.pow(2) + h.pow(2) + w_po2_diff + h_po2_diff;

                /*
                info!(
                    "Frames_per_row = {} gives (w,h) = {:?} and (w_diff, h_diff) = {:?} and cost = {}",
                    params.frames_per_row,
                    (w, h),
                    (w_po2_diff, h_po2_diff),
                    cost
                );*/

                cost
            })
            .unwrap();
        let duration = start.elapsed();

        info!(
            "Time elapsed in get_optimal_spritesheet_params() is: {:?}",
            duration
        );
        info!("Closest = {:?}", closest_to_po2);

        closest_to_po2
    }

    fn get_frame_size() -> (usize, usize) {
        do_with_image_holder(|imageholder, _owner| {
            let frames = &imageholder.output_frames;

            assert!(frames
                .iter()
                .all(|f| (f.width(), f.height()) == (frames[0].width(), frames[0].height())));

            let frame_width = frames[0].width() as usize;
            let frame_height = frames[0].height() as usize;

            (frame_width, frame_height)
        })
        .unwrap()
    }

    fn get_spritesheet_size(spritesheet_params: &SpritesheetGenerationParams) -> (usize, usize) {
        do_with_image_holder(|imageholder, _owner| {
            let frames = &imageholder.output_frames;
            let frames_per_row = spritesheet_params.frames_per_row.min(frames.len());

            let (frame_width, frame_height) = Self::get_frame_size();
            let rows = (frames.len() as f32 / frames_per_row as f32).ceil() as usize;

            let final_img_width = frame_width * frames_per_row;
            let final_img_height = frame_height * rows;

            (final_img_width, final_img_height)
        })
        .unwrap()
    }

    #[export]
    fn generate_spritesheet(
        &mut self,
        _owner: &Base,
        spritesheet_params: SpritesheetGenerationParams,
    ) -> Ref<ImageTexture, Unique> {
        do_with_image_holder(|imageholder, _owner| {
            let frames = &imageholder.output_frames;
            let frames_per_row = spritesheet_params.frames_per_row.min(frames.len());

            let (frame_width, frame_height) = Self::get_frame_size();
            let (final_img_width, final_img_height) =
                Self::get_spritesheet_size(&spritesheet_params);

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
