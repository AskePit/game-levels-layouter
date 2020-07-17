#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Point
{
    pub x: usize,
    pub y: usize,
}

impl Point
{
	pub fn new<T: num::cast::AsPrimitive<usize>>(x: T, y: T) -> Self {
		Point {
			x: x.as_(),
			y: y.as_(),
		}
	}

	pub fn get_neighbour(&self, x_diff: i8, y_diff: i8) -> Option<Self> {
		if x_diff < 0 && (self.x < (-x_diff as usize)) {
			return None;
		}
	
		if y_diff < 0 && (self.y < (-y_diff as usize)) {
			return None;
		}

		Some(Point::new(self.x as isize + x_diff as isize, self.y as isize + y_diff as isize))
	}
}

impl Default for Point
{
	fn default() -> Self {
		Point::new(0, 0)
	}
}

#[derive(Default, Clone, Copy, Debug)]
pub struct BBox
{
	pub bl: Point,
	pub tr: Point,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Color
{
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Color
{
	pub const BLACK: Self = Color::new(0, 0, 0);

	pub const fn new(r: u8, g: u8, b:u8) -> Self {
		Color {r, g, b}
	}
}

use std::collections::HashMap;
pub type NeighboursMap = HashMap<Point, Vec<Point>>;
pub type ComplexGeometry = Vec<Point>;

#[derive(Clone, Debug)]
pub enum ShapeGeometry
{
	Box(BBox),
	Complex(ComplexGeometry),
}

impl Default for ShapeGeometry
{
	fn default() -> Self {
		ShapeGeometry::Box(BBox::default())
	}
}

#[derive(Clone, Debug)]
pub struct Shape
{
	pub color: Color,
	pub geometry: ShapeGeometry,
}

impl Default for Shape
{
	fn default() -> Self {
		Shape {
			color: Color::BLACK,
			geometry: ShapeGeometry::default()
		}
	}
}
