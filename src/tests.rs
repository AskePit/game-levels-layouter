use crate::types::*;
use crate::utils;

use std::iter::FromIterator;

#[test]
fn test_black_image() {
	let layout = utils::get_shapes_layout("assets/black_sample.png").unwrap();

	assert_eq!(layout.shapes.len(), 1);
	assert_eq!(layout.shapes.get(&Color::BLACK).unwrap().len(), 10);

	// shape types count
	{
		let mut points_count: usize = 0;
		let mut boxes_count: usize = 0;
		let mut complex_count: usize = 0;

		for shape in layout.shapes.get(&Color::BLACK).unwrap() {
			match shape {
				Shape::Pixel(_) => points_count += 1,
				Shape::Box(_) => boxes_count += 1,
				Shape::Complex(_) => complex_count += 1,
			}
		}

		assert_eq!(points_count, 3);
		assert_eq!(boxes_count, 2);
		assert_eq!(complex_count, 5);
	}
}

#[test]
fn test_inner_complex_geometry() {
	let layout = utils::get_shapes_layout("assets/black_sample.png").unwrap();

	let mut count = 0usize;

	for shape in layout.shapes.get(&Color::BLACK).unwrap() {
		if let Shape::Complex(geom) = &shape {
			let outer_bbox = geom.get_outer_bbox();
			let points = geom.get_points();
			let boxes = geom.get_bboxes();

			// big circle
			if *outer_bbox == BBox::new_xy(2, 1, 8, 7) {
				assert_eq!(points.len(), 0);
				assert_eq!(boxes.len(), 5);

				assert!(boxes.contains(&BBox::new_xy(4, 1, 6, 7)));
				assert!(boxes.contains(&BBox::new_xy(2, 3, 2, 5)));
				assert!(boxes.contains(&BBox::new_xy(3, 2, 3, 6)));
				assert!(boxes.contains(&BBox::new_xy(7, 2, 7, 6)));
				assert!(boxes.contains(&BBox::new_xy(8, 3, 8, 5)));

				count += 1;
			}

			// bottom shape
			else if *outer_bbox == BBox::new_xy(0, 13, 15, 15) {
				assert_eq!(points.len(), 0);
				assert_eq!(boxes.len(), 6);

				assert!(boxes.contains(&BBox::new_xy(4, 13, 8, 15)));
				assert!(boxes.contains(&BBox::new_xy(2, 14, 3, 15)));
				assert!(boxes.contains(&BBox::new_xy(0, 15, 1, 15)));
				assert!(boxes.contains(&BBox::new_xy(9, 14, 12, 15)));
				assert!(boxes.contains(&BBox::new_xy(13, 15, 14, 15)));
				assert!(boxes.contains(&BBox::new_xy(15, 14, 15, 15)));

				count += 1;
			}

			// snake shape
			else if *outer_bbox == BBox::new_xy(9, 10, 15, 12) {
				assert_eq!(points.len(), 3);
				assert_eq!(boxes.len(), 4);

				assert!(boxes.contains(&BBox::new_xy(10, 11, 10, 12)));
				assert!(boxes.contains(&BBox::new_xy(12, 11, 12, 12)));
				assert!(boxes.contains(&BBox::new_xy(14, 11, 14, 12)));
				assert!(boxes.contains(&BBox::new_xy(15, 10, 15, 11)));

				assert!(points.contains(&Point::new(9, 11)));
				assert!(points.contains(&Point::new(11, 12)));
				assert!(points.contains(&Point::new(13, 11)));

				count += 1;
			}

			// Г shape
			else if *outer_bbox == BBox::new_xy(7, 9, 8, 11) {
				assert_eq!(points.len(), 1);
				assert_eq!(boxes.len(), 1);

				assert!(boxes.contains(&BBox::new_xy(7, 9, 7, 11)));

				assert!(points.contains(&Point::new(8, 9)));

				count += 1;
			}

			// last shape
			else if *outer_bbox == BBox::new_xy(10, 1, 11, 3) {
				assert_eq!(points.len(), 0);
				assert_eq!(boxes.len(), 2);

				assert!(boxes.contains(&BBox::new_xy(10, 2, 10, 3)));
				assert!(boxes.contains(&BBox::new_xy(11, 1, 11, 3)));

				count += 1;
			}
		}
	}

	assert_eq!(count, 5);
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

		let bbox = shape.get_outer_bbox();

		assert_eq!(bbox.min, *min);
		assert_eq!(bbox.max, *max);
	}
}

#[test]
fn test_complex_geometry_try_get_as_bbox() {
	let data: Vec<(Vec<Point>, Option<BBox>)> = vec![
		(
			vec![Point::new(2, 0),	Point::new(2, 1)],
			Some(BBox::new_xy(2, 0, 2, 1))
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
			Some(BBox::new_xy(1, 10, 3, 11))
		),
		(
			vec![Point::new(1, 10)],
			Some(BBox::new_xy(1, 10, 1, 10))
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
