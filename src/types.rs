use image::RgbaImage;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Point
{
    pub x: usize,
    pub y: usize,
}

impl Point
{
	pub fn new<T: num::cast::AsPrimitive<usize>>(x: T, y: T) -> Point {
		Point {
			x: x.as_(),
			y: y.as_(),
		}
	}

	pub fn get_neighbour(&self, x_diff: i8, y_diff: i8) -> Option<Point> {
		if x_diff < 0 && (self.x < (-x_diff as usize)) {
			return None;
		}
	
		if y_diff < 0 && (self.y < (-y_diff as usize)) {
			return None;
		}

		Some(Point::new(self.x as isize + x_diff as isize, self.y as isize + y_diff as isize))
	}
}

pub struct BBox
{
	pub bl: Point,
	pub tr: Point,
}

use std::collections::HashMap;
pub type NeighboursMap = HashMap<Point, Vec<Point>>;

pub struct Color
{
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

fn is_solid_color(rgba: &[u8; 4]) -> bool {
	let is_transparent = rgba[3] != 255;
	let is_white = is_transparent || (rgba[0] == 255 && rgba[1] == 255 && rgba[2] == 255);

	!is_white
}

pub fn is_solid_coord(img: &RgbaImage, point: &Point) -> bool {
	let (width, height) = img.dimensions();
	let width = width as usize;
	let height = height as usize;

	let (x, y) = (point.x, point.y);

	if x >= width || y >= height {
		return false;
	}

	let xy = img.get_pixel(x as u32, y as u32);
	is_solid_color(&xy.0)
}

pub fn process_neighbour(p: &Point, x_diff: i8, y_diff: i8, v: &mut Vec<Point>, img: &RgbaImage) {
	if let Some(neighbour) = p.get_neighbour(x_diff, y_diff) {
		if is_solid_coord(img, &neighbour) {
			v.push(neighbour);
		}
	}
}