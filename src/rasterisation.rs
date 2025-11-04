use crate::num::Num;
use crate::colour::Colour;
use crate::linear_algebra::*;
use crate::frame_buffer::{FrameBuffer, FrameBufferTrait};

pub enum WindingOrder {
    CCW,
    CW
}

#[derive(Clone, Copy)]
pub struct VertexAttributes {
    pub colour: Colour,
}

impl VertexAttributes {
    fn new() -> Self {
        VertexAttributes { 
            colour: Colour::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vertex<T: Num> {
    pub vertex: Vec3<T>,
    pub attributes: VertexAttributes,
}

impl<T: Num> Vertex<T> {
    pub fn new(vertex: Vec3<T>, attributes: VertexAttributes) -> Self {
        Vertex {vertex, attributes}
    }
}

#[derive(Clone, Copy)]
pub struct Triangle<T: Num> {
    pub v0: Vertex<T>,
    pub v1: Vertex<T>,
    pub v2: Vertex<T>,
}

#[derive(Debug)]
pub struct Range<T: Num> {
    min: T,
    max: T,
}

#[derive(Debug)]
pub struct BoundingBox<T: Num> {
    x: Range<T>,
    y: Range<T>,
}

impl<T: Num> Range<T> {
    fn find_min_max<const L: usize>(array: [&T; L], type_min: T, type_max: T) -> Self {
        let mut min = type_max; 
        let mut max = type_min; 

        for element in array {
            if *element > max {
                max = *element;
            }

            if *element < min {
                min = *element;
            }
        }

        Range {min, max}
    }
}

impl Triangle<f32> {
    pub fn transform_this_triangle(&mut self, transformation_matrix: &Matrix44) {
        let vertices = [&mut self.v0.vertex, &mut self.v1.vertex, &mut self.v2.vertex];

        for vertex in vertices {
            *vertex = vertex.homogeneous_mult_matrix(transformation_matrix);
        }
        
    }

    pub fn transform_triangle(&self, transformation_matrix: &Matrix44) -> Triangle<f32> {
        let vertices = [&self.v0.vertex, &self.v1.vertex, &self.v2.vertex];

        let mut new_triangle = Triangle {
            v0: Vertex::new(Vec3::splat(0.0), self.v0.attributes),
            v1: Vertex::new(Vec3::splat(0.0), self.v1.attributes),
            v2: Vertex::new(Vec3::splat(0.0), self.v2.attributes),
        };

        let new_vertices = [&mut new_triangle.v0.vertex, &mut new_triangle.v1.vertex, &mut new_triangle.v2.vertex];

        for (i, vertex) in vertices.iter().enumerate() {
            *new_vertices[i] = vertex.homogeneous_mult_matrix(transformation_matrix);
        }
        
        new_triangle
    }

    pub fn get_bounding_box(&self) -> BoundingBox<f32> {
        let vertices_x = [&self.v0.vertex.x, &self.v1.vertex.x, &self.v2.vertex.x];
        let vertices_y = [&self.v0.vertex.y, &self.v1.vertex.y, &self.v2.vertex.y];

        BoundingBox {
            x: Range::find_min_max(vertices_x, f32::NEG_INFINITY, f32::INFINITY),
            y: Range::find_min_max(vertices_y, f32::NEG_INFINITY, f32::INFINITY),
        }
    }

    // Divide vertex attributes by their z coordiante for perspective correct interpolation
    fn divide_attributes(&self) -> [VertexAttributes; 3] {
        let mut new_attributes = [VertexAttributes::new(), VertexAttributes::new(), VertexAttributes::new()];

        for (i, vertex) in [&self.v0, &self.v1, &self.v2].iter().enumerate() {
            let zdiv = 1.0 / vertex.vertex.z;
            let colour = &vertex.attributes.colour;

            new_attributes[i].colour = colour.multiply_float(zdiv);

        }

        new_attributes
    }

}

// Return true if this edge is a top or left edge
// Parameters v0 and v1 are the vertices of the edge 
// The edge vector goes from v0 to v1
fn is_top_left<T: Num>(v0: &Vec3<T>, v1: &Vec3<T>, winding: &WindingOrder) -> bool {
    let (is_top_edge, is_left_edge) = match winding {
        WindingOrder::CCW => (v0.y == v1.y && v0.x > v1.x, v0.y > v1.y),
        WindingOrder::CW => (v0.y == v1.y && v0.x < v1.x, v0.y < v1.y)
    };
   
    is_top_edge || is_left_edge
}


// Computes the edge function given two vertices and a point
// Changes sign if winding order is CCW
fn edge_fn<T: Num>(v0: &Vec3<T>, v1: &Vec3<T>, p: &Vec3<T>, winding: &WindingOrder) -> T {
    let result = ((p.x - v0.x) * (v1.y - v0.y)) - ((p.y - v0.y) * (v1.x - v0.x));

    match winding {
        WindingOrder::CCW => return -result,
        WindingOrder::CW => return result,
    }
}

// Draws a traingle to the frame buffer
pub fn rasterise_triangle<T: FrameBufferTrait>(triangle: &Triangle<f32>, frame_buffer: &mut FrameBuffer<T>, winding: &WindingOrder) {

    // Add bias to corresponding edge function functions
    // This avoids calculating if edges are top / left multiple times
    // https://youtu.be/k5wtuKWmV48?si=x79mf8aEe-YOoNeP&t=4197
    let bias0 = if is_top_left(&triangle.v0.vertex, &triangle.v1.vertex, winding) {0.0} else {-1.0};
    let bias1 = if is_top_left(&triangle.v1.vertex, &triangle.v2.vertex, winding) {0.0} else {-1.0};
    let bias2 = if is_top_left(&triangle.v2.vertex, &triangle.v0.vertex, winding) {0.0} else {-1.0};

    // Calculate delta w's 
    // This works because each edge function changes by the same amount across a row or a column
    // https://youtu.be/k5wtuKWmV48?si=qOR57hqKZoHXAVYW&t=6290
    let delta_w0_x = triangle.v0.vertex.y - triangle.v1.vertex.y;
    let delta_w1_x = triangle.v1.vertex.y - triangle.v2.vertex.y;
    let delta_w2_x = triangle.v2.vertex.y - triangle.v0.vertex.y;

    let delta_w0_y = triangle.v1.vertex.x - triangle.v0.vertex.x;
    let delta_w1_y = triangle.v2.vertex.x - triangle.v1.vertex.x;
    let delta_w2_y = triangle.v0.vertex.x - triangle.v2.vertex.x;

    let bounding_box = triangle.get_bounding_box();
    let px_bounding_box = BoundingBox {
        x: Range {min: bounding_box.x.min.floor() as i32, max: bounding_box.x.max.ceil() as i32},
        y: Range {min: bounding_box.y.min.floor() as i32, max: bounding_box.y.max.ceil() as i32},
    };

    // Add 0.5 to check pixel center
    let start_point = Vec3::new(bounding_box.x.min.floor() + 0.5, bounding_box.y.min.floor() + 0.5, 0.0);

    // Calculate starting edge functions do apply deltas to as we move through the bounding box
    let mut col_w0 = edge_fn(&triangle.v0.vertex, &triangle.v1.vertex, &start_point, winding) + bias0;
    let mut col_w1 = edge_fn(&triangle.v1.vertex, &triangle.v2.vertex, &start_point, winding) + bias1;
    let mut col_w2 = edge_fn(&triangle.v2.vertex, &triangle.v0.vertex, &start_point, winding) + bias2;
    let double_triangle_area = col_w0 + col_w1 + col_w2; 

    // Precompute 1/z's for perspective correct barycentric interpolation 
    let div_zs: [f32; 3] = [1.0 / triangle.v0.vertex.z, 1.0 / triangle.v1.vertex.z, 1.0 / triangle.v2.vertex.z];

    // Divide 
    let divided_attributes = triangle.divide_attributes();

    for x in px_bounding_box.x.min..px_bounding_box.x.max {

        let mut w0 = col_w0;
        let mut w1 = col_w1;
        let mut w2 = col_w2;

        for y in px_bounding_box.y.min..px_bounding_box.y.max {
            let mut point_overlap = true;
            point_overlap &= w0 >= 0.0;
            point_overlap &= w1 >= 0.0;
            point_overlap &= w2 >= 0.0;

            w0 += delta_w0_y;
            w1 += delta_w1_y;
            w2 += delta_w2_y;

            if !point_overlap {
                continue;
            }

            // Barycentric coordinates
            let l0 = w1 / double_triangle_area;
            let l1 = w2 / double_triangle_area;
            let l2 = w0 / double_triangle_area;

            // Get perspective correct interpolated z
            let interpolated_z = 1.0 / (div_zs[0] * l0 + div_zs[1] * l1 + div_zs[2] * l2);

            // Interpolate pixel colour using barycentric coorindates (perspective correct)
            let pixel_colour = (
                divided_attributes[0].colour.multiply_float(l0) +
                divided_attributes[1].colour.multiply_float(l1) +
                divided_attributes[2].colour.multiply_float(l2)
            ).multiply_float(interpolated_z);

            let _ = frame_buffer.write_buf(x as usize, y as usize, &pixel_colour);
        }

        col_w0 += delta_w0_x;
        col_w1 += delta_w1_x;
        col_w2 += delta_w2_x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}

