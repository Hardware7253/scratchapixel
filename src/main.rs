// This project uses a right handed coordinate system where z points into the screen
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use std::{thread, time};

pub mod linear_algebra;
pub mod camera;

use linear_algebra::*;

fn main() {

    let delay_time = time::Duration::from_millis(150);

    let mut angle: f32 = 0.5;
    let z_offset = 2.0;
    let depth = 4.0;

    let cube_points = vec![
        Vec3::new(-1.0, 1.0, z_offset),
        Vec3::new(1.0, 1.0, z_offset),
        Vec3::new(-1.0, -1.0, z_offset),
        Vec3::new(1.0, -1.0, z_offset),
        Vec3::new(-1.0, 1.0, z_offset - depth),
        Vec3::new(1.0, 1.0, z_offset - depth),
        Vec3::new(-1.0, -1.0, z_offset - depth),
        Vec3::new(1.0, -1.0, z_offset - depth),
    ];

    while true {
        

        // Rotate cube about the z axis
        let cube_transformation =  Matrix44::new([
            [angle.cos(), 0.0, -angle.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [angle.sin(), 0.0, angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let connected_vertices = vec![(0, 1), (1, 3), (2, 3), (0, 2), (4, 5), (5, 7), (6, 7), (4, 6), (0, 4), (1, 5), (2, 6), (3, 7)];

        let mut camera = camera::Camera::new();
        camera.transformation_matrix =  Matrix44::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, -10.0, 1.0],
        ]);

        let mut raster_points = Vec::new();
        for point in &cube_points {
            let point = point.mult_matrix(&cube_transformation);
            let raster_point = camera.point_to_raster(&point);
            raster_points.push(raster_point.0);
        }

        let mut data = Data::new();
        for (v1, v2) in connected_vertices {
            let (p1, p2) = (&raster_points[v1], &raster_points[v2]);
            data = data.move_to((p1.x, p1.y))
                    .line_to((p2.x, p2.y));
        }

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.1)
            .set("d", data);

        let document = Document::new()
            .set("viewBox", (0, 0, camera.screen_size.x, camera.screen_size.y))
            .add(path);

        svg::save("image.svg", &document).unwrap();
        thread::sleep(delay_time);
        angle += 0.1;
        break;

    }
}
