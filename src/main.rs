// This project uses a right handed coordinate system where z points into the screen

pub mod num;
pub mod colour;
pub mod frame_buffer;

pub mod linear_algebra;
pub mod math_helpers;

pub mod camera;
pub mod rasterisation;

use colour::*;
use linear_algebra::*;
use frame_buffer::*;
use rasterisation::*;
// use num::Num;

use minifb::{Key, Window, WindowOptions};

const WINDING_ORDER: WindingOrder = WindingOrder::CCW;

const DRAW_WIDTH: usize = 128;
const DRAW_HEIGHT: usize = 128;

// Convert pixel coordinates to array index
fn convert_coordinates(px_x: usize, px_y: usize, width_px: usize, height_px: usize) -> Result<usize, FrameBufError> {
    if (px_x >= width_px || px_y >= height_px) {
        return Err(frame_buffer::FrameBufError::PixelOutsideBuf);
    }

    let write_y = height_px - px_y - 1;
    let index = px_x + (write_y * width_px);
    Ok(index)
}

impl<const L: usize> FrameBufferTrait for [u32; L] {

    fn write_buf(&mut self, px_x: usize, px_y: usize, colour: &Colour8, width_px: usize, height_px: usize) -> Result<(), FrameBufError> {
        let index = convert_coordinates(px_x, px_y, width_px, height_px)?;
        let bytes: [u8; 4] = [colour.alpha, colour.red, colour.green, colour.blue]; // minifb doesn't use the alpha channel
        self[index] = u32::from_be_bytes(bytes);

        Ok(())
    }


    fn read_buf(&self, px_x: usize, px_y: usize, width_px: usize, height_px: usize) -> Result<Colour8, FrameBufError> {
        let index = convert_coordinates(px_x, px_y, width_px, height_px)?;
        let colour = self[index];
        let colour_bytes: [u8; 4] = u32::to_be_bytes(colour);

        let colour8 = Colour8 {
            red: colour_bytes[1],
            green: colour_bytes[2],
            blue: colour_bytes[3],
            alpha: colour_bytes[0],
        };

        Ok(colour8)
    }
}

fn main() {
    let mut frame_buffer = FrameBuffer::new(DRAW_WIDTH, DRAW_HEIGHT, [0; DRAW_WIDTH * DRAW_HEIGHT]);

    let v0 = Vertex {
        vertex: Vec3::new(40.0, 8.0, 0.0),  // already Vec3
        attributes: VertexAttributes { colour: RED },
    };

    let v1 = Vertex {
        vertex: Vec3::new(100.0, 60.0, 0.0),  // already Vec3
        attributes: VertexAttributes { colour: GREEN },
    };

    let v2 = Vertex {
        vertex: Vec3::new(20.0, 100.0, 0.0),  // Convert Vec2 to Vec3
        attributes: VertexAttributes { colour: BLUE },
    };

    let triangle1 = Triangle {
        v0,
        v1,
        v2,
    };

    let v0 = Vertex {
        vertex: Vec3::new(40.0, 8.0, 0.0),  // Convert Vec2 to Vec3
        attributes: VertexAttributes { colour: BLUE },
    };

    let v2 = Vertex {
        vertex: Vec3::new(100.0, 60.0, 0.0),  // Convert Vec2 to Vec3
        attributes: VertexAttributes { colour: RED },
    };

    let v1 = Vertex {
        vertex: Vec3::new(120.0, 5.0, 0.0),  // Convert Vec2 to Vec3
        attributes: VertexAttributes { colour: RED },
    };

    let triangle2 = Triangle {
        v0,
        v1,
        v2,
    };


    let v0 = Vertex {
        vertex: Vec3::new(-40.0f32, -40.0, 0.0),
        attributes: VertexAttributes {colour: RED},
    };

    let v1 = Vertex {
        vertex: Vec3::new(60.0f32, 5.0, 0.0),
        attributes: VertexAttributes {colour: GREEN},
    };

    let v2 = Vertex {
        vertex: Vec3::new(-5.0f32, 50.0, 0.0),
        attributes: VertexAttributes {colour: BLUE},
    };

    let mut triangle3 = Triangle {
        v0,
        v1,
        v2,
    };

    let mut window_options = WindowOptions::default();
    window_options.scale_mode = minifb::ScaleMode::Stretch;
    window_options.scale = minifb::Scale::X8;

    let mut window = Window::new(
        "Test - ESC to exit",
        DRAW_WIDTH,
        DRAW_HEIGHT,
        window_options
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(24);

    let angle: f32 = 0.03;

    // Rotate about the z axis
    let transformation_matrix =  Matrix44::new([
        [angle.cos(), -angle.sin(), 0.0, 0.0],
        [angle.sin(), angle.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    let centre = Vec3::new(60.0, 60.0, 0.0);
    let translation_matrix=  Matrix44::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [centre.x, centre.y, centre.z, 1.0],
    ]);

    let mut count = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        frame_buffer.clear_buf();

        triangle3.transform_this_triangle(&transformation_matrix);
        rasterise_triangle(&triangle3.transform_triangle(&translation_matrix), &mut frame_buffer, &WINDING_ORDER);

        // Top left check
        // rasterise_triangle(&triangle1, &mut frame_buffer, &WINDING_ORDER);
        // if count % 2 == 0 {
        //     rasterise_triangle(&triangle2, &mut frame_buffer, &WINDING_ORDER);
        // }

        count += 1;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&frame_buffer.buf, DRAW_WIDTH, DRAW_HEIGHT)
            .unwrap();

    }
}




// fn main() {
//     let camera_transformation_matrix =  Matrix44::new([
//         [1.0, 0.0, 0.0, 0.0],
//         [0.0, 1.0, 0.0, 0.0],
//         [0.0, 0.0, 1.0, 0.0],
//         [0.0, 0.0, -10.0, 1.0],
//     ]);

//     let camera = camera::Camera::new(camera_transformation_matrix, Vec2::new(100, 100), 15.0, Vec2::new(36.0, 24.0), 0.1, 100.0, camera::FitResolutionGate::Fill);

//     let v0 = Vec2::new(0.0, 0.0);
//     let v1 = Vec2::new(1.0, 1.0);
//     let p = Vec2::new(0.0, -2.0);

//     // println!("{}", math_helpers::compute(v0, v1, p));

        
// }
