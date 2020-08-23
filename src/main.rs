
mod types;
mod utils;


#[cfg(test)]
mod tests;

fn main() {
    let shapes = utils::get_shapes_layout("assets/color_sample.png").unwrap();

    println!("{:#?}", shapes);
}
