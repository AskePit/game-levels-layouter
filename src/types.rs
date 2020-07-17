#![allow(dead_code)]

pub use std::collections::HashMap;
pub use std::collections::HashSet;

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

#[derive(Eq, PartialEq, Default, Clone, Copy, Debug)]
pub struct BBox
{
	pub min: Point,
	pub max: Point,
}

impl BBox
{
	pub fn new(min: &Point, max: &Point) -> Self {
		Self {
			min: *min,
			max: *max
		}
	}

	pub fn is_point_in(&self, point: &Point) -> bool {
		point.x >= self.min.x && point.x <= self.max.x && point.y >= self.min.y && point.y <= self.max.y
	}
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

pub type NeighboursMap = HashMap<Point, Vec<Point>>;

#[derive(Clone, Debug, Default)]
pub struct ComplexGeometry
{
	points: HashSet<Point>,
	bbox: BBox,
}

impl ComplexGeometry
{
	pub fn new(points: HashSet<Point>) -> Self {
		let mut n = Self {
			points,
			..Self::default()
		};
		n._calc_bbox();

		n
	}

	fn _calc_bbox(&mut self) {
		if self.points.is_empty() {
			return;
		}

		let mut min_x: usize = 9999;
		let mut max_x: usize = 0;
		let mut min_y: usize = 9999;
		let mut max_y: usize = 0;

		for point in &self.points {
			min_x = min_x.min(point.x);
			max_x = max_x.max(point.x);
			min_y = min_y.min(point.y);
			max_y = max_y.max(point.y);
		}

		self.bbox.min = Point::new(min_x, min_y);
		self.bbox.max = Point::new(max_x, max_y);
	}

	pub fn get_points(&self) -> &HashSet<Point> {
		&self.points
	}

	pub fn get_bbox(&self) -> &BBox {
		&self.bbox
	}

	pub fn try_get_as_bbox(&self) -> Option<BBox> {
		for x in self.bbox.min.x..=self.bbox.max.x {
			for y in self.bbox.min.y..=self.bbox.max.y {
				let p = Point::new(x, y);

				if !self.points.contains(&p) {
					return None;
				}
			}
		}

		Some(self.bbox)
	}
}

#[derive(Clone, Debug)]
pub enum ShapeGeometry
{
	Pixel(Point),
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

impl Shape
{
	pub fn new(color: Color, geometry: ShapeGeometry) -> Self {
		Self {
			color,
			geometry: match &geometry {
				ShapeGeometry::Complex(complex_geometry) => {
					if complex_geometry.get_points().len() == 1 {
						let point = complex_geometry.get_points().into_iter().cloned().next().unwrap();
						ShapeGeometry::Pixel(point)
					} else if let Some(bbox) = complex_geometry.try_get_as_bbox() {
						ShapeGeometry::Box(bbox)
					} else {
						geometry
					}
				},
				ShapeGeometry::Box(bbox) => {
					if bbox.min == bbox.max {
						ShapeGeometry::Pixel(bbox.min)
					} else {
						geometry
					}
				},
				_ => geometry,
			}
		}
	}
}