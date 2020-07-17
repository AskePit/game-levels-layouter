
use super::*;
use types::*;

use std::iter::FromIterator;

#[test]
fn test_black_image() {
	let shapes = utils::get_shapes("assets/black_sample.png").unwrap();

	assert_eq!(shapes.len(), 10);

	// shape types count
	{
		let mut points_count: usize = 0;
		let mut boxes_count: usize = 0;
		let mut complex_count: usize = 0;

		for shape in &shapes {
			match shape.geometry {
				ShapeGeometry::Pixel(_) => points_count += 1,
				ShapeGeometry::Box(_) => boxes_count += 1,
				ShapeGeometry::Complex(_) => complex_count += 1,
			}
		}

		assert_eq!(points_count, 3);
		assert_eq!(boxes_count, 2);
		assert_eq!(complex_count, 5);
	}

	// complex geometry points count
	{
		let mut counts: HashMap<usize, usize> = HashMap::new();

		for shape in &shapes {
			if let ShapeGeometry::Complex(geom) = &shape.geometry {
				*counts.entry(geom.get_points().len()).or_insert(0) += 1;
			}
		}

		let samples: [(usize, usize); 5] = [(4, 1), (5, 1), (11, 1), (33, 1), (37, 1)];

		for sample in samples.iter() {
			let (pixels, count) = sample;

			assert_eq!(*counts.get(pixels).unwrap(), *count);
		}
	}
}

#[test]
fn test_complex_geometry_bbox() {

	let data: Vec<(Vec<Point>, Point, Point)> = vec![
		(
			vec![],
			Point::new(0, 0),
			Point::new(0, 0)
		),
		(
			vec![Point::new(2, 1)],
			Point::new(2, 1),
			Point::new(2, 1)
		),
		(
			vec![Point::new(2, 0),	Point::new(2, 1)],
			Point::new(2, 0),
			Point::new(2, 1)
		),
		(
			vec![Point::new(0, 2), Point::new(0, 3), Point::new(1, 3), Point::new(2, 3)],
			Point::new(0, 2),
			Point::new(2, 3)
		),
		(
			vec![
				Point::new(1, 4),
				Point::new(1, 5),
				Point::new(1, 6),
				Point::new(2, 3),
				Point::new(2, 4),
				Point::new(2, 5),
				Point::new(2, 6),
				Point::new(2, 7),
				Point::new(3, 2),
				Point::new(3, 3),
				Point::new(3, 4),
				Point::new(3, 5),
				Point::new(3, 6),
				Point::new(3, 7),
				Point::new(3, 8),
				Point::new(4, 2),
				Point::new(4, 3),
				Point::new(4, 4),
				Point::new(4, 5),
				Point::new(4, 6),
				Point::new(4, 7),
				Point::new(4, 8),
				Point::new(5, 2),
				Point::new(5, 3),
				Point::new(5, 4),
				Point::new(5, 5),
				Point::new(5, 6),
				Point::new(5, 7),
				Point::new(5, 8),
				Point::new(6, 3),
				Point::new(6, 4),
				Point::new(6, 5),
				Point::new(6, 6),
				Point::new(6, 7),
				Point::new(7, 4),
				Point::new(7, 5),
				Point::new(7, 6),
			],
			Point::new(1, 2),
			Point::new(7, 8)
		),
	];

	for (points, min, max) in &data
	{
		let set: HashSet<Point> = HashSet::from_iter(points.into_iter().cloned());
		let shape = ComplexGeometry::new(set.clone());

		let bbox = shape.get_bbox();

		assert_eq!(bbox.min, *min);
		assert_eq!(bbox.max, *max);
	}
}

#[test]
fn test_complex_geometry_try_get_as_bbox() {
	let data: Vec<(Vec<Point>, Option<BBox>)> = vec![
		(
			vec![Point::new(2, 0),	Point::new(2, 1)],
			Some(BBox::new(&Point::new(2, 0), &Point::new(2, 1)))
		),
		(
			vec![Point::new(0, 2), Point::new(0, 3), Point::new(1, 3), Point::new(2, 3)],
			None
		),
		(
			vec![Point::new(1, 11), Point::new(2, 10), Point::new(2, 11), Point::new(3, 10), Point::new(3, 11)],
			None
		),
		(
			vec![Point::new(1, 10), Point::new(1, 11), Point::new(2, 10), Point::new(2, 11), Point::new(3, 10), Point::new(3, 11)],
			Some(BBox::new(&Point::new(1, 10), &Point::new(3, 11)))
		),
		(
			vec![Point::new(1, 10)],
			Some(BBox::new(&Point::new(1, 10), &Point::new(1, 10)))
		),
		(
			vec![Point::new(7, 9), Point::new(8, 9), Point::new(7, 10), Point::new(7, 11)],
			None
		),
	];

	for (points, sample_bbox) in &data
	{
		let set: HashSet<Point> = HashSet::from_iter(points.into_iter().cloned());
		let shape = ComplexGeometry::new(set.clone());

		let bbox = shape.try_get_as_bbox();

		assert_eq!(bbox, *sample_bbox);
	}
}
