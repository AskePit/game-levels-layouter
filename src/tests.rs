
use super::*;
use types::*;

use std::collections::HashMap;

#[test]
fn test_black_image() {
	let shapes = utils::get_shapes("assets/black_sample.png").unwrap();

	assert_eq!(shapes.len(), 10);

	{
		let mut counts: HashMap<usize, usize> = HashMap::new();

		for shape in &shapes {
			if let ShapeGeometry::Complex(geom) = &shape.geometry {
				*counts.entry(geom.len()).or_insert(0) += 1;
			}
		}

		let samples: [(usize, usize); 8] = [(1, 3), (2, 1), (4, 1), (5, 1), (11, 1), (15, 1), (33, 1), (37, 1)];

		for sample in samples.iter() {
			let (pixels, count) = sample;

			assert_eq!(*counts.get(pixels).unwrap(), *count);
		}
		assert_eq!(*counts.get(&5).unwrap(), 1);
		assert_eq!(*counts.get(&15).unwrap(), 1);
		assert_eq!(*counts.get(&33).unwrap(), 1);
		assert_eq!(*counts.get(&37).unwrap(), 1);
		
	}
}