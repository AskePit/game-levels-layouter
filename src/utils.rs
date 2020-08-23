use image::RgbaImage;

use std::collections::HashMap;

use crate::types::{
    NeighboursMap,
    Point,
    Shape,
    Color,
    ComplexGeometry,
    BBox,
};

fn is_point_in_image(img: &RgbaImage, point: &Point) -> bool {
    let (width, height) = img.dimensions();
	let width = width as usize;
	let height = height as usize;

	let (x, y) = (point.x, point.y);

	x < width && y < height
}

fn is_solid_color(rgba: &[u8; 4]) -> bool {
	let is_transparent = rgba[3] != 255;
	let is_white = is_transparent || (rgba[0] == 255 && rgba[1] == 255 && rgba[2] == 255);

	!is_white
}

fn is_solid_coord(img: &RgbaImage, point: &Point) -> bool {
	if !is_point_in_image(img, point) {
        return false
    }

	let xy = img.get_pixel(point.x as u32, point.y as u32);
	is_solid_color(&xy.0)
}

fn is_same_color(img: &RgbaImage, p1: &Point, p2: &Point) -> bool {
    if !is_point_in_image(img, p1) || !is_point_in_image(img, p2) {
        return false
    }
	img.get_pixel(p1.x as u32, p1.y as u32) == img.get_pixel(p2.x as u32, p2.y as u32)
}

fn process_neighbour(p: &Point, x_diff: i8, y_diff: i8, v: &mut Vec<Point>, img: &RgbaImage) {
	if let Some(neighbour) = p.get_neighbour(x_diff, y_diff) {
		if is_same_color(img, &p, &neighbour) {
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
                
                let pixel = img.get_pixel(p.x as u32, p.y as u32);
                let color = Color::new(pixel[0], pixel[1], pixel[2]);

                neighbours.insert(p, (color, Vec::new()));

                let v = &mut neighbours.get_mut(&p).unwrap().1;

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

fn collect_complex_shape(start_point: &Point, neighbours: &NeighboursMap, processed: &mut HashSet<Point>, shape_points: &mut HashSet<Point>) {
    if !neighbours.contains_key(start_point) {
        return;
    }

    if processed.contains(start_point) {
        return;
    }

    processed.insert(*start_point);
    shape_points.insert(*start_point);

    for near in &neighbours.get(start_point).unwrap().1 {
        collect_complex_shape(near, neighbours, processed, shape_points);
    }
}

pub fn get_shapes(img_path: &str) -> Result<HashMap<Color, Vec<Shape>>, image::ImageError> {

    let img = image::open(img_path)?.into_rgba();
    let neighbours = get_neighbours_map(&img);

    Ok(get_shapes_by_neighbour_points(neighbours))
}

pub fn get_shapes_by_neighbour_points(neighbours: NeighboursMap) -> HashMap<Color, Vec<Shape>> {

    let mut shapes: HashMap<Color, Vec<Shape>> = HashMap::new();
    let mut processed: HashSet<Point> = HashSet::new();

    for p in neighbours.iter() {
        let (point, nears_info) = p;
        let (color, _nears) = nears_info;
        if !processed.contains(point) {
            let mut geometry_points: HashSet<Point> = HashSet::new();

            collect_complex_shape(point, &neighbours, &mut processed, &mut geometry_points);

            let mut shape =
                if geometry_points.len() == 1 {
                    Shape::Pixel(*geometry_points.iter().next().unwrap())
                } else if let Some(bbox) = are_points_is_bbox(&geometry_points) {
                    Shape::Box(bbox)
                } else {
                    let complex_geometry = ComplexGeometry::new(geometry_points);
                    Shape::Complex(complex_geometry)
                };

            shape.simplify();

            if !shapes.contains_key(color) {
                shapes.insert(*color, Vec::new());
            }

            shapes.get_mut(color).unwrap().push(shape);
        }
    }

    shapes
}

fn are_points_is_bbox(points: &HashSet<Point>) -> Option<BBox> {
    let bbox = calc_bbox_by_points(points);

    if bbox.get_points_count() == points.len() {
        Some(bbox)
    } else {
        None
    }
}

pub fn calc_bbox_by_points(points: &HashSet<Point>) -> BBox {

    let mut bbox = BBox::default();

    if points.is_empty() {
        return bbox;
    }

    let mut min_x: usize = 9999;
    let mut max_x: usize = 0;
    let mut min_y: usize = 9999;
    let mut max_y: usize = 0;

    for point in points {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    bbox.min = Point::new(min_x, min_y);
    bbox.max = Point::new(max_x, max_y);

    bbox
}
