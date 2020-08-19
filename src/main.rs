
mod types;
mod utils;


#[cfg(test)]
mod tests;

fn main() {
    let shapes = utils::get_shapes("assets/color_sample.png").unwrap();

    println!("{:#?}", shapes);
}
