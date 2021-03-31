#![feature(array_chunks)]

#[macro_use]
extern crate approx;

mod about_popup;
mod datatypes;
mod frame;
mod global_holder;
mod image_holder;
mod image_processor;
mod image_saver;
mod logging;
mod spritesheet_generator;
mod utility;

use gdnative::prelude::*;
use log::*;
use logging::init_logging;

use crate::{
    about_popup::AboutPopup, global_holder::GlobalHolder, image_holder::ImageHolder,
    image_processor::ImageProcessor, image_saver::ImageSaver,
    spritesheet_generator::SpritesheetGenerator,
};

fn init(handle: InitHandle) {
    match init_logging() {
        Ok(()) => {}
        Err(err) => {
            godot_print!("Failed to initialize logger: {}", err);
        }
    }

    handle.add_class::<ImageSaver>();
    handle.add_class::<ImageHolder>();
    handle.add_class::<ImageProcessor>();
    handle.add_class::<GlobalHolder>();
    handle.add_class::<SpritesheetGenerator>();
    handle.add_class::<AboutPopup>();

    info!(
        "\n---\n{} v{}\n{} ({})\nBuild date: {} {}\n---",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        env!("VERGEN_CARGO_PROFILE"),
        env!("VERGEN_BUILD_DATE"),
        env!("VERGEN_BUILD_TIME"),
    );
}

godot_init!(init);
