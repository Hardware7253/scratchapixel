// This project uses a right handed coordinate system where z points into the screen
pub mod linear_algebra;
pub mod camera;

fn main() {
    let matrix = linear_algebra::Matrix44::identity();
    println!("{}", 1);
}
