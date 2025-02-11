use crate::linear_algebra::*;

// Camera points in the negative z direction 
pub struct Camera {
    pub transformation_matrix: Matrix44, // World to camera matrix
    pub canvas_size: Vec3<f32>, // Canvas size in camera space
    pub screen_size: Vec3<u32>  // Screen size in raster space
}


impl Camera {

    // Makes a new camera centered at the world origin
    // Canvas is assumed to be one unit away from the camera
    pub fn new() -> Self {
        Camera {
            transformation_matrix: Matrix44::identity(),
            canvas_size: Vec3::new(3.0, 3.0, 0.0),
            screen_size: Vec3::new(100, 100, 0),
        }
    }

    // Converts a point from world space to raster space
    // Returns a None value if the converted point lies outside the cameras view
    pub fn point_to_raster(&self, world_point: &Vec3<f32>) -> (Vec3<u32>, bool) {

        // Convert point from world to camera coordinates
        let camera_point = world_point.homogeneous_mult_matrix(&self.transformation_matrix);

        // Project point onto canvas using z divide
        let proj_x = camera_point.x / -camera_point.z; // Negative sign accounts for camera looking in the negative z direction
        let proj_y = camera_point.y / camera_point.z;

        // Convert canvas coordinates to normalised device coordinates
        let ndc_x = proj_x / self.canvas_size.x + 0.5;
        let ndc_y = proj_y / self.canvas_size.y + 0.5;

        let outside_canvas = ndc_x > 1.0 || ndc_x < 0.0 || ndc_y > 1.0 || ndc_y < 0.0;

        // Convert NDC to raster coordinates
        let raster_coordinates: Vec3<u32> = Vec3::new(
            (ndc_x * self.screen_size.x as f32).floor() as u32,
            (ndc_y * self.screen_size.y as f32).floor() as u32,
            0
        );

        (raster_coordinates, outside_canvas)
    }
}