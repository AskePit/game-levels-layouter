mod types;
mod utils;

pub use crate::types::*;

pub fn get_shapes(img_path: &str) -> Result<HashMap<Color, Vec<Shape>>, image::ImageError> {
	utils::get_shapes(img_path)
}