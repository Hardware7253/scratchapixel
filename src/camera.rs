use crate::linear_algebra::*;

// Camera points in the negative z direction 
pub struct Camera {
    // World to camera matrix
    pub transformation_matrix: Matrix44,

    //                 www, hhh
    pub canvas_size: (f32, f32), // Canvas size in camera space
    pub screen_size: (u32, u32)  // Screen size in raster space
}


impl Camera {

    // Makes a new camera centered at the world origin
    // Canvas is assumed to be one unit away from the camera
    pub fn new() -> Self {
        Camera {
            transformation_matrix: Matrix44::identity(),
            canvas_size: (3.0, 3.0),
            screen_size: (100, 100),
        }
    }

    // Converts a point from world space to raster space
    // Returns a None value if the converted point lies outside the cameras view
    pub fn point_to_raster(&self, world_point: &Vec3) -> Option<u32> {

        // Convert point from world to camera coordinates
        let camera_point = world_point.homogeneous_mult_matrix(&self.transformation_matrix);

        let proj_x = camera_point.x / -camera_point.z;
        let proj_y = camera_point.y / -camera_point.z;

        let ndc_x = proj_x / self.canvas_size.0 + 0.5;
        let ndc_y = proj_y / self.canvas_size.0 + 0.5;

        if ndc_x > 1.0 || ndc_x < 0.0 || ndc_y > 1.0 || ndc_y < 1.0 {
            return None;
        }





        None
    }

}