use image::RgbaImage;

mod types;

use crate::types::{
    NeighboursMap,
    Point,
    Shape,
    ShapeGeometry,
    ComplexGeometry,
};

fn is_solid_color(rgba: &[u8; 4]) -> bool {
	let is_transparent = rgba[3] != 255;
	let is_white = is_transparent || (rgba[0] == 255 && rgba[1] == 255 && rgba[2] == 255);

	!is_white
}

fn is_solid_coord(img: &RgbaImage, point: &Point) -> bool {
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

fn process_neighbour(p: &Point, x_diff: i8, y_diff: i8, v: &mut Vec<Point>, img: &RgbaImage) {
	if let Some(neighbour) = p.get_neighbour(x_diff, y_diff) {
		if is_solid_coord(img, &neighbour) {
			v.push(neighbour);
		}
	}
}

fn get_neighbours_map(img: &RgbaImage) -> NeighboursMap {
    let (width, height) = img.dimensions();

    let mut neighbours = NeighboursMap::new();

    for y in 0..height {
        for x in 0..width {
            let p = Point::new(x, y);

            if is_solid_coord(&img, &p) {
                
                neighbours.insert(p, Vec::new());

                let v = neighbours.get_mut(&p).unwrap();

                process_neighbour(&p, -1, 0, v, &img);
                process_neighbour(&p, 1, 0, v, &img);
                process_neighbour(&p, 0, -1, v, &img);
                process_neighbour(&p, 0, 1, v, &img);
            }
        }
    }

    neighbours
}

use std::collections::HashSet;

fn collect_complex_shape_rec(start_point: &Point, neighbours: &NeighboursMap, processed: &mut HashSet<Point>, shape_points: &mut ComplexGeometry) {
    if !neighbours.contains_key(start_point) {
        return;
    }

    if processed.contains(start_point) {
        return;
    }

    processed.insert(*start_point);
    shape_points.push(*start_point);

    for near in neighbours.get(start_point).unwrap() {
        collect_complex_shape_rec(near, neighbours, processed, shape_points);
    }
}

fn get_shapes(neighbours: NeighboursMap) -> Vec<Shape> {

    let mut shapes = Vec::new();
    let mut processed: HashSet<Point> = HashSet::new();

    for p in neighbours.iter() {
        let (point, _nears) = p;
        if !processed.contains(point) {
            let mut complex_geometry = ComplexGeometry::new();

            collect_complex_shape_rec(point, &neighbours, &mut processed, &mut complex_geometry);

            let shape = Shape {
                geometry: ShapeGeometry::Complex(complex_geometry),
                ..Shape::default()
            };
            shapes.push(shape);
        }
    }

    shapes
}

fn main() {
    let img = image::open("assets/black_sample.png").unwrap().into_rgba();
    let neighbours = get_neighbours_map(&img);
    let shapes = get_shapes(neighbours);

    println!("{:#?}", shapes);
}
