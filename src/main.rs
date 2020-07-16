extern crate image;

mod types;

use crate::types::{
    is_solid_coord,
    process_neighbour,
    NeighboursMap,
    Point,
};

fn main() {
    let img = image::open("assets/black_sample.png").unwrap().into_rgba();
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
}
