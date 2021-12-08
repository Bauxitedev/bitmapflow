use std::{f32::consts::PI, ops::Deref};

use bracket_color::prelude::*;
use gdnative::{
    api::{Image, ImageTexture},
    core_types::{FromVariant, FromVariantError, ToVariant, TypedArray, Variant},
    prelude::*,
};
use image::{Rgba, RgbaImage};
use imgref::Img;
use opencv::{
    core::{Vec2f, Vec3b, CV_8UC3},
    prelude::*,
};

use crate::datatypes::BVector2;

#[derive(Clone, Debug)]
pub struct Frame(pub FrameInner);
pub type Frames = Vec<Frame>;
type FrameInner = RgbaImage;

//Little hack so we can pretend a Frame is an Img<Vec<...>> and call its methods directly
impl Deref for Frame {
    type Target = FrameInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromVariant for Frame {
    fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
        let image_texture =
            variant
                .try_to_object::<ImageTexture>()
                .ok_or(FromVariantError::InvalidInstance {
                    expected: "ImageTexture",
                })?;
        let image_texture = unsafe { image_texture.assume_safe() };

        let data = image_texture.get_data().ok_or_else(|| {
            FromVariantError::Custom("Couldn't read data from ImageTexture".to_string())
        })?;
        let data = unsafe { data.assume_safe() };
        let (w, h, data) = (
            data.get_width() as usize,
            data.get_height() as usize,
            data.get_data(),
        );

        let frame = RgbaImage::from_raw(w as u32, h as u32, data.read().to_vec()).unwrap();
        Ok(Frame(frame))
    }
}

impl ToVariant for Frame {
    fn to_variant(&self) -> Variant {
        let frame = &self.0;
        let img_tex = ImageTexture::new();
        let img = Image::new();

        let mut data: Vec<u8> = vec![];
        for rgba in frame.pixels() {
            data.push(rgba[0]);
            data.push(rgba[1]);
            data.push(rgba[2]);
            data.push(rgba[3]);
        }

        img.create_from_data(
            frame.width() as i64,
            frame.height() as i64,
            false,
            Image::FORMAT_RGBA8,
            TypedArray::from_vec(data),
        );
        img_tex.create_from_image(img, 0);
        Variant::from_object(img_tex)
    }
}

impl From<Img<Vec<rgb::RGBA<u8>>>> for Frame {
    fn from(img: Img<Vec<rgb::RGBA<u8>>>) -> Self {
        Frame(RgbaImage::from_fn(
            img.width() as u32,
            img.height() as u32,
            |x, y| {
                let rgba: rgb::RGBA<u8> = img[(x, y)];
                image::Rgba([rgba.r, rgba.g, rgba.b, rgba.a])
            },
        ))
    }
}

//NOTE: this conversion is lossy (alpha is thrown away)
impl From<&Frame> for Mat {
    fn from(input: &Frame) -> Self {
        let width = input.width();
        let height = input.height();

        let mut mat = unsafe { Mat::new_rows_cols(height as i32, width as i32, CV_8UC3).unwrap() };

        for x in 0..width {
            for y in 0..height {
                let [r, g, b, a] = input[(x, y)].0;

                let bgr_color = if a < 30 {
                    Vec3b::from([0, 0, 0]) //TODO using pure black for (almost) transparent pixels, may go wrong for dark sprites? Try using a color that does not appear in the input images
                } else {
                    Vec3b::from([b, g, r]) //Note this is BGR, not RGB
                };

                **(mat
                    .at_2d_mut(y as i32, x as i32)
                    .as_mut()
                    .expect("input frame addressing failed")) = bgr_color;
            }
        }

        mat
    }
}

pub fn flow_mat_to_frame(input: &Mat) -> Frame {
    let w = input.size().unwrap().width as u32;
    let h = input.size().unwrap().height as u32;

    assert!(input.depth() == opencv::core::CV_32F);
    assert!(input.channels() == 2);

    let to_byte = |f: f32| (f * 255.0).round() as u8;

    let inner = RgbaImage::from_fn(w, h, |x, y| {
        let color: Vec2f = *input
            .at_2d(y as i32, x as i32)
            .expect("Expected a 2-channel 32-bit floating point image");

        let flow: BVector2 = color.into();

        if flow.x.is_finite() && flow.y.is_finite() {
            let magnitude = flow.length() * 0.3;
            let angle_normalized = flow.angle_from_x_axis().positive().get() / (PI * 2.0);
            let col = HSV::from_f32(angle_normalized, 1.0, magnitude.clamp(0.0, 1.0)).to_rgb();
            Rgba::from([to_byte(col.r), to_byte(col.g), to_byte(col.b), 255])
        } else {
            Rgba::from([0, 0, 0, 0]) //if NaN output a transparent pixel
        }
    });

    Frame(inner)
}

//TODO: impl From<Ref<ImageTexture, Shared>> for Frame?
pub fn texture_to_image(img: Ref<ImageTexture, Shared>) -> RgbaImage {
    let img = unsafe { img.assume_safe() };

    let data = img.get_data().unwrap();
    let data = unsafe { data.assume_safe() };
    let data = data.get_data();
    let data = data.read();

    RgbaImage::from_vec(
        img.get_width() as u32,
        img.get_height() as u32,
        data.to_vec(),
    )
    .unwrap()
}
