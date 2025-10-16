use crate::linear_algebra::*;

// Camera points in the negative z direction 
// https://www.scratchapixel.com/images/cameras/canvascoordinates4.png?
// All physical dimensions are defined in millimeters
pub struct Camera {
    pub transformation_matrix: Matrix44, // World to camera matrix

    pub image_size: Vec2<i32>,  // Image size in pixels 

    // Distance between eye and image plane in a pinhole camera (used to calculate angle of view)
    // Not the same as the distance to the virtual cameras canvas 
    pub focal_length: f32, 

    pub camera_aperture: Vec2<f32>, // Physical dimensions of film used in a real camera (used to calculate angle of view)

    // Z coordinates of the near and far clipping planes
    // The virtual canvas is placed at the near clipping plane
    pub z_near: f32,
    pub z_far: f32,

    pub fit_resolution_gate: FitResolutionGate,

    // Angle of view for the camera
    horizontal_angle_of_view: f32,
    vertical_angle_of_view: f32,

    canvas_size: Vec2<f32>, // X and Y dimensions of the canvas 
    screen_window: (Vec2<f32>, Vec2<f32>), // Bottom left and top right coordinates of the canvas edges respectively

    film_gate_aspect_ratio: f32, // Calculated from the cameras aperture
    resolution_gate_aspect_ratio: f32, // Calculated from the image size
}

pub enum FitResolutionGate {
    Fill, // Fit resolution gate within film gate (shrink film to match canvas)
    Overscan, // Fit film gate within resolution gate (grow film to match canvas)
}

pub enum ProjectionError {
    PointCLipped,
    PointOutsideCanvas,
}

impl Camera {

    // Makes a new camera centered at the world origin
    // Canvas is assumed to be one unit away from the camera
    pub fn new( transformation_matrix: Matrix44, 
        image_size: Vec2<i32>, 
        focal_length: f32, 
        camera_aperture: Vec2<f32>, 
        z_near: f32, 
        z_far: f32,
        fit_resolution_gate: FitResolutionGate,
    ) -> Self {
        let horizontal_angle_of_view = 2.0 * f32::atan((camera_aperture.x / 2.0) / focal_length);
        let vertical_angle_of_view = 2.0 * f32::atan((camera_aperture.y / 2.0) / focal_length);

        // Calculate aspect ratios
        let film_gate_aspect_ratio = camera_aperture.x / camera_aperture.y;
        let resolution_gate_aspect_ratio = image_size.x as f32 / image_size.y as f32;

        // Determine canvas x and y scale factors depending on fit mode
        // I still don't understand this part that well I pretty much yoinked it from here https://www.scratchapixel.com/lessons/3d-basic-rendering/3d-viewing-pinhole-camera/implementing-virtual-pinhole-camera.html
        let (scale_x, scale_y) = match fit_resolution_gate {
            FitResolutionGate::Fill => {
                if film_gate_aspect_ratio > resolution_gate_aspect_ratio {
                    (resolution_gate_aspect_ratio / film_gate_aspect_ratio, 1.0)
                } else {
                    (1.0, film_gate_aspect_ratio / resolution_gate_aspect_ratio)
                }
            },
            FitResolutionGate::Overscan => {
                if film_gate_aspect_ratio > resolution_gate_aspect_ratio {
                    (1.0, film_gate_aspect_ratio / resolution_gate_aspect_ratio)
                } else {
                    (resolution_gate_aspect_ratio / film_gate_aspect_ratio, 1.0)
                }
            },
        };

        // Calculate canvas size
        let canvas_height = (camera_aperture.y / 2.0 / focal_length) * z_near; // Using similiar triangles 
        let canvas_size = Vec2::new(canvas_height * film_gate_aspect_ratio * scale_x, canvas_height * scale_y);

        // Calculate screen window
        let bottom_left = Vec2::new(canvas_size.x / -2.0, canvas_size.y / -2.0);
        let top_right = Vec2::new(-bottom_left.x, -bottom_left.y);
        let screen_window = (bottom_left, top_right);

        Camera {
            transformation_matrix,
            image_size,
            focal_length,
            camera_aperture,
            z_near,
            z_far,
            fit_resolution_gate,
            horizontal_angle_of_view,
            vertical_angle_of_view,
            canvas_size,
            screen_window,
            film_gate_aspect_ratio,
            resolution_gate_aspect_ratio,
        }
    }

    // Converts a point from world space to screen space
    pub fn point_to_screen(&self, world_point: &Vec3<f32>) -> Result<Vec3<f32>, ProjectionError> {

        // Convert point from world to camera coordinates
        let camera_point = world_point.homogeneous_mult_matrix(&self.transformation_matrix);

        if camera_point.z < self.z_near || camera_point.z > self.z_far {
            return Err(ProjectionError::PointCLipped);
        }

        // Project point onto canvas using z divide
        // Place canvas at z_near
        let proj_x = camera_point.x / -camera_point.z * self.z_near; // Negative sign accounts for camera looking in the negative z direction
        let proj_y = camera_point.y / camera_point.z * self.z_near;

        Ok(Vec3::new(proj_x, proj_y, camera_point.z))
    }

    // Converts a point from screen space to raster space
    pub fn screen_to_raster(&self, screen_point: &Vec3<f32>) -> Result<Vec2<i32>, ProjectionError> {

        // Convert canvas coordinates to normalised device coordinates
        let ndc_x = screen_point.x / self.canvas_size.x + 0.5;
        let ndc_y = screen_point.y / self.canvas_size.y + 0.5;

        // Check point is inside the canvas
        if ndc_x > 1.0 || ndc_x < 0.0 || ndc_y > 1.0 || ndc_y < 0.0 {
            return Err(ProjectionError::PointOutsideCanvas);
        }

        // Convert NDC to raster coordinates
        let raster_coordinates: Vec2<i32> = Vec2::new(
            (ndc_x * self.image_size.x as f32).floor() as i32,
            (ndc_y * self.image_size.y as f32).floor() as i32,
        );

        Ok(raster_coordinates)
    }

    // Converts a point from world space to raster space
    pub fn point_to_raster(&self, world_point: &Vec3<f32>) -> Result<Vec2<i32>, ProjectionError> {
        let screen_point = self.point_to_screen(world_point)?;
        self.screen_to_raster(&screen_point)
    }
}
