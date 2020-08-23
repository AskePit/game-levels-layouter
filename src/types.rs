#![allow(dead_code)]

pub use std::collections::HashMap;
pub use std::collections::HashSet;

use crate::utils;

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

#[derive(Eq, PartialEq, Hash, Default, Clone, Copy, Debug)]
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

	pub fn new_xy(min_x: usize, min_y: usize, max_x: usize, max_y: usize) -> Self {
		Self {
			min: Point::new(min_x, min_y),
			max: Point::new(max_x, max_y),
		}
	}

	pub fn is_point(&self) -> bool {
		self.min == self.max
	}

	pub fn contains(&self, point: &Point) -> bool {
		point.x >= self.min.x &&
		point.x <= self.max.x &&
		point.y >= self.min.y &&
		point.y <= self.max.y
	}

	pub fn get_width(&self) -> usize {
		self.max.x - self.min.x + 1
	}

	pub fn get_height(&self) -> usize {
		self.max.y - self.min.y + 1
	}

	pub fn get_square(&self) -> usize {
		self.get_width() * self.get_height()
	}

	pub fn get_points_count(&self) -> usize {
		self.get_square()
	}
}

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

pub type NeighboursMap = HashMap<Point, (Color, Vec<Point>)>;

#[derive(Clone, Debug, Default)]
pub struct ComplexGeometry
{
	bboxes: HashSet<BBox>,
	points: HashSet<Point>,
	outer_bbox: BBox,
}

#[derive(Clone, Debug, Default)]
struct SplittedComplexGeometry
{
	bboxes: HashSet<BBox>,
	points: HashSet<Point>,
}

impl SplittedComplexGeometry
{
	pub fn new(points: &HashSet<Point>, outer_bbox: &BBox) -> Self {
		let mut obj = SplittedComplexGeometry::default();
		obj._init(points, outer_bbox);
		obj
	}

	pub fn merge(&mut self, other: SplittedComplexGeometry) {
		self.points.extend(other.points);
		self.bboxes.extend(other.bboxes);
	}

	fn _init(&mut self, points: &HashSet<Point>, outer_bbox: &BBox) {
		if points.is_empty() {
			return;
		}

		// find the heightiest column of pixels in all shape
		// then we'll find the largest bbox of all shape with this height
		let mut min_y = outer_bbox.max.y;
		let mut max_y = 0usize;
		{
			let mut max_height = 0usize;

			for x in outer_bbox.min.x ..= outer_bbox.max.x {
				let mut column_started = false;
				let mut min_column_y: Option<usize> = None;
				let mut max_column_y: Option<usize> = None;
				let mut column_height = 0usize;

				for y in outer_bbox.min.y ..= outer_bbox.max.y {
					if points.contains(&Point::new(x, y)) {
						if !column_started {
							column_started = true;
							min_column_y.replace(y);
							column_height = 1;
						} else {
							max_column_y.replace(y);
							column_height += 1;
						}
					} else {
						if min_column_y.is_some() {
							if column_height > max_height {
								max_height = column_height;
								min_y = min_column_y.unwrap();
								max_y = max_column_y.unwrap_or(min_y);
							}
						}
						column_height = 0;
						column_started = false;
						min_column_y = None;
						max_column_y = None;
					}
				}

				if column_height > max_height {
					max_height = column_height;
					min_y = min_column_y.unwrap();
					max_y = max_column_y.unwrap_or(min_y);
				}
			}
		}

		// find heightest bboxes
		let min_y = min_y;
		let max_y = max_y;

		let mut heightiest_bboxes = Vec::new();

		let mut min_x: Option<usize> = None;
		let mut max_x: Option<usize> = None;

		for x in outer_bbox.min.x ..= outer_bbox.max.x {
			let mut match_height: usize = 0;
			for y in min_y ..= max_y {
				let p = Point::new(x, y);
				if points.contains(&p) {
					match_height += 1;
				} else {
					break;
				}
			}

			if match_height == (max_y - min_y + 1) {
				if min_x.is_none() {
					min_x.replace(x);
				} else {
					max_x.replace(x);
				}
			} else {
				if min_x.is_some() {
					if max_x.is_none() {
						max_x.replace(min_x.unwrap());
					}

					heightiest_bboxes.push(BBox::new_xy(min_x.take().unwrap(), min_y, max_x.take().unwrap(), max_y));
				} else {
					min_x = None;
					max_x = None;
				}
			}
		}

		if min_x.is_some() {
			if max_x.is_none() {
				max_x.replace(min_x.unwrap());
			}

			heightiest_bboxes.push(BBox::new_xy(min_x.take().unwrap(), min_y, max_x.take().unwrap(), max_y));
		}

		for bbox in &heightiest_bboxes {
			if bbox.is_point() {
				self.points.insert(bbox.min);
			} else {
				self.bboxes.insert(*bbox);
			}
		}

		// process rest shapes that did not belong to heightiest bboxes
		{
			let rest_shapes = Self::_split_points_by_bboxes(points.clone(), heightiest_bboxes);
			for shape in &rest_shapes {
				match shape {
					Shape::Pixel(point) => {self.points.insert(*point);},
					Shape::Box(bbox) => {self.bboxes.insert(*bbox);},
					Shape::Complex(geom) => {self.merge(geom.copy_inner_geometry());},
				}
			}
		}
	}

	fn _split_points_by_bboxes(mut points: HashSet<Point>, heightiest_bboxes: Vec<BBox>) -> Vec<Shape> {
		for bbox in heightiest_bboxes {
			for x in bbox.min.x ..= bbox.max.x {
				for y in bbox.min.y ..= bbox.max.y {
					points.remove(&Point::new(x, y));
				}
			}
		}

		if points.is_empty() {
			return Vec::new();
		}

		let neighbours_map = Self::_get_neighbours_map(&points);
		let shapes = utils::get_shapes_by_neighbour_points(neighbours_map);

		shapes.into_iter().flat_map(|x| x.1).map(|x| x).collect()
	}

	fn _get_neighbours_map(points: &HashSet<Point>) -> NeighboursMap {
		let mut neighbours = NeighboursMap::new();

		let bbox = utils::calc_bbox_by_points(points);

		for y in bbox.min.y ..= bbox.max.y {
			for x in bbox.min.x ..= bbox.max.x {
				let p = Point::new(x, y);

				if points.contains(&p) {
					neighbours.insert(p, (Color::BLACK, Vec::new()));

					let v = &mut neighbours.get_mut(&p).unwrap().1;

					if let Some(neighbour) = p.get_neighbour(-1, 0) {
						if points.contains(&neighbour) {
							v.push(neighbour);
						}
					}

					if let Some(neighbour) = p.get_neighbour(1, 0) {
						if points.contains(&neighbour) {
							v.push(neighbour);
						}
					}

					if let Some(neighbour) = p.get_neighbour(0, -1) {
						if points.contains(&neighbour) {
							v.push(neighbour);
						}
					}

					if let Some(neighbour) = p.get_neighbour(0, 1) {
						if points.contains(&neighbour) {
							v.push(neighbour);
						}
					}
				}
			}
		}

		neighbours
	}

	pub fn get_bboxes(&self) -> &HashSet<BBox> {
		&self.bboxes
	}

	pub fn get_points(&self) -> &HashSet<Point> {
		&self.points
	}
}

impl ComplexGeometry
{
	pub fn new(points: HashSet<Point>) -> Self {
		let mut n = Self {
			..Self::default()
		};
		n._calc_outer_bbox(&points);
		n._calc_inner_geometries(&points);

		n
	}

	fn _calc_outer_bbox(&mut self, points: &HashSet<Point>) {
		self.outer_bbox = utils::calc_bbox_by_points(points);
	}

	fn _calc_inner_geometries(&mut self, points: &HashSet<Point>) {
		let inner_geometry = SplittedComplexGeometry::new(points, &self.outer_bbox);
		self.bboxes = inner_geometry.bboxes;
		self.points = inner_geometry.points;
	}

	pub fn contains(&self, point: &Point) -> bool {
		for bbox in &self.bboxes {
			if bbox.contains(point) {
				return true;
			}
		}

		for p in &self.points {
			if p == point {
				return true;
			}
		}

		false
	}

	pub fn get_bboxes(&self) -> &HashSet<BBox> {
		&self.bboxes
	}

	pub fn get_points(&self) -> &HashSet<Point> {
		&self.points
	}

	pub fn get_outer_bbox(&self) -> &BBox {
		&self.outer_bbox
	}

	pub fn try_get_as_bbox(&self) -> Option<BBox> {
		let outer = &self.outer_bbox;
		for x in outer.min.x ..= outer.max.x {
			for y in outer.min.y ..= outer.max.y {
				let p = Point::new(x, y);

				if !self.contains(&p) {
					return None;
				}
			}
		}

		Some(self.outer_bbox)
	}

	pub fn try_get_as_point(&self) -> Option<Point> {
		let outer = &self.outer_bbox;
		if outer.is_point() {
			return Some(outer.min);
		}

		None
	}

	fn copy_inner_geometry(&self) -> SplittedComplexGeometry {
		SplittedComplexGeometry {
			bboxes: self.bboxes.clone(),
			points: self.points.clone()
		}
	}
}

#[derive(Clone, Debug)]
pub enum Shape
{
	Pixel(Point),
	Box(BBox),
	Complex(ComplexGeometry),
}

impl Default for Shape
{
	fn default() -> Self {
		Shape::Box(BBox::default())
	}
}

impl Shape
{
	pub fn simplify(&mut self) {
		*self = match self {
			Shape::Complex(complex_geometry) => {
				if let Some(point) = complex_geometry.try_get_as_point() {
					Shape::Pixel(point)
				} else if let Some(bbox) = complex_geometry.try_get_as_bbox() {
					Shape::Box(bbox)
				} else {
					return;
				}
			},
			Shape::Box(bbox) => {
				if bbox.is_point() {
					Shape::Pixel(bbox.min)
				} else {
					return;
				}
			},
			_ => return,
		}
	}

	pub fn get_simplified(&self) -> Self {
		let mut cloned = self.clone();
		cloned.simplify();
		cloned
	}
}

#[derive(Debug)]
pub struct ShapesLayout
{
	pub shapes: HashMap<Color, Vec<Shape>>,
	pub color_dependencies: HashMap<Color, HashSet<Color>>
}