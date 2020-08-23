#![allow(dead_code)]

mod types;
mod utils;

pub use crate::types::*;

pub fn get_shapes_layout(img_path: &str) -> Result<ShapesLayout, image::ImageError> {
	utils::get_shapes_layout(img_path)
}