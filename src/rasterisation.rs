use crate::num::Num;
use crate::colour::Colour8;
use crate::linear_algebra::*;
use crate::frame_buffer::{FrameBuffer, FrameBufferTrait};

pub enum WindingOrder {
    CCW,
    CW
}

pub struct VertexAttributes {
    pub colour: Colour8,
}

pub struct Vertex<T: Num> {
    pub vertex: Vec2<T>,
    pub attributes: VertexAttributes,
}

pub struct Triangle<T: Num> {
    pub v0: Vertex<T>,
    pub v1: Vertex<T>,
    pub v2: Vertex<T>,
}

impl Triangle<f32> {

    pub fn transform_triangle(&mut self, transformation_matrix: &Matrix44) {
        let vertices = [&mut self.v0.vertex, &mut self.v1.vertex, &mut self.v2.vertex];

        for vertex in vertices {
            let vertex_vector = Vec3::from_vec2(vertex, 0.0);
            let new_vector = vertex_vector.homogeneous_mult_matrix(transformation_matrix);
            *vertex = Vec2::from_vec3(&new_vector);
        }
        
    }
}

// Return true if this edge is a top or left edge
// Parameters v0 and v1 are the vertices of the edge 
// The edge vector goes from v0 to v1
fn is_top_left<T: Num>(v0: &Vec2<T>, v1: &Vec2<T>, winding: &WindingOrder) -> bool {
    let (is_top_edge, is_left_edge) = match winding {
        WindingOrder::CCW => (v0.y == v1.y && v0.x > v1.x, v0.y > v1.y),
        WindingOrder::CW => (v0.y == v1.y && v0.x < v1.x, v0.y < v1.y)
    };
   
    is_top_edge || is_left_edge
}


// Computes the edge function given two vertices and a point
// Changes sign if winding order is CCW
fn edge_fn<T: Num>(v0: &Vec2<T>, v1: &Vec2<T>, p: &Vec2<T>, winding: &WindingOrder) -> T {
    let result = ((p.x - v0.x) * (v1.y - v0.y)) - ((p.y - v0.y) * (v1.x - v0.x));

    match winding {
        WindingOrder::CCW => return -result,
        WindingOrder::CW => return result,
    }
}

// Tests if a point is contained in a traingle
// Returns true if that point is in the triangle
// Also returns barycentric coefficients 
fn test_point(triangle: &Triangle<i32>, double_triangle_area: i32, point: Vec2<i32>, winding: &WindingOrder) -> (bool, f32, f32, f32) {
    let w0 = edge_fn(&triangle.v0.vertex, &triangle.v1.vertex, &point, winding);
    let w1 = edge_fn(&triangle.v1.vertex, &triangle.v2.vertex, &point, winding);
    let w2 = edge_fn(&triangle.v2.vertex, &triangle.v0.vertex, &point, winding);

    let mut edge = false;
    let mut top_left = true;

    if w0 == 0 {
        top_left &= is_top_left(&triangle.v0.vertex, &triangle.v1.vertex, winding);
        edge = true;
    }

    if w1 == 0 {
        top_left &= is_top_left(&triangle.v1.vertex, &triangle.v2.vertex, winding);
        edge = true;
    }

    if w2 == 0 {
        top_left &= is_top_left(&triangle.v2.vertex, &triangle.v0.vertex, winding);
        edge = true;
    }

    let mut point_overlap = true;
    point_overlap &= w0 >= 0;
    point_overlap &= w1 >= 0;
    point_overlap &= w2 >= 0;

    if edge {
        point_overlap &= top_left;
    }

    let l0 = w1 as f32 / double_triangle_area as f32;
    let l1 = w2 as f32 / double_triangle_area as f32;
    let l2 = w0 as f32 / double_triangle_area as f32;

    (point_overlap, l0, l1, l2)
}

// Draws a traingle to the frame buffer
pub fn rasterise_triangle<T: FrameBufferTrait>(triangle: &Triangle<i32>, frame_buffer: &mut FrameBuffer<T>, winding: &WindingOrder) {

    let double_triangle_area = edge_fn(&triangle.v0.vertex, &triangle.v1.vertex, &triangle.v2.vertex, winding);

    // Use bounding box later
    for x in 0..frame_buffer.width_px {
        for y in 0..frame_buffer.height_px {
            let this_point = Vec2::new(x as i32, y as i32);
            let (point_overlap, l0, l1, l2) = test_point(&triangle, double_triangle_area, this_point, winding);

            if !point_overlap {
                continue;
            }

            // Interpolate pixel colour using barycentric coorindates
            let pixel_colour = triangle.v0.attributes.colour.multiply_float(l0) +
                               triangle.v1.attributes.colour.multiply_float(l1) +
                               triangle.v2.attributes.colour.multiply_float(l2);

            let _ = frame_buffer.write_buf(x, y, &pixel_colour);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_top_left() {
        let v0 = Vec2::new(0.0, 0.0);
        let v1 = Vec2::new(1.0, 1.0);
        assert_eq!(is_top_left(&v0, &v1, &WindingOrder::CW), true);


        let v0 = Vec2::new(1.0, 1.0);
        let v1 = Vec2::new(3.0, -1.0);
        assert_eq!(is_top_left(&v0, &v1, &WindingOrder::CCW), true);
        assert_eq!(is_top_left(&v0, &v1, &WindingOrder::CW), false);
    }

}

