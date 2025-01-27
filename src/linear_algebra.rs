//This file contains definitions and implementations for a vector and a matrix type

#[derive(Debug, PartialEq)]
pub struct Vec3 {
   pub x: f32,
   pub y: f32,
   pub z: f32, 
}

impl Vec3 {
    // Vector constructor
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            x,
            y,
            z,
        }
    }

    // Construct a vector with all it's fields having the same value 
    pub fn splat(d: f32) -> Self {
        Vec3 {
            x: d.clone(),
            y: d.clone(),
            z: d.clone(),
        }
    }

    // Return dot product of two vectors
    pub fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    // Return cross product of two vectors
    pub fn cross(&self, v: &Vec3) -> Vec3 {
       Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
       ) 
    }

    // Return the length of the vector
    pub fn len(&self) -> f32 {
       f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    // Normalise length of vector so it's one unit long
    pub fn normalise(&mut self) {
        let normalisation_constant = 1.0 / self.len();
        
        self.x = self.x * normalisation_constant;
        self.y = self.y * normalisation_constant;
        self.z = self.z * normalisation_constant;
    }
    
    // Multiply matrices [1x3] x [3x3] = [1x3]
    // The Matrix44 is treated as a 3x3 matrix to perform this multiplication
    pub fn mult_matrix(&self, matrix: &Matrix44) -> Vec3 {
        let mut vec_array: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            vec_array[i] = self.x * matrix.0[0][i] +
                           self.y * matrix.0[1][i] +
                           self.z * matrix.0[2][i];
        }

        Vec3::new(vec_array[0], vec_array[1], vec_array[2])
    }

    // Multiply matrices [1x4] x [4x4] = [1x4]
    // The homogenous coordinate of the input vector is implied to be one
    // The homogeneous output coordinates are normalised so a Vec3 can be returned
    pub fn homogeneous_mult_matrix(&self, matrix: &Matrix44) -> Vec3 {
        let mut vec_array: [f32; 4] = [0.0; 4];
        for i in 0..4 {
            vec_array[i] = self.x * matrix.0[0][i] +
                           self.y * matrix.0[1][i] +
                           self.z * matrix.0[2][i] +
                          /* 1 * */ matrix.0[3][i];
        }

        // Convert homogeneous coordinates back to cartesian
        vec_array[0] /= vec_array[3];
        vec_array[1] /= vec_array[3];
        vec_array[2] /= vec_array[3];

        Vec3::new(vec_array[0], vec_array[1], vec_array[2])
    }
}

type MatrixArray = [[f32; 4]; 4];
const ZERO_MATRIX: MatrixArray = [
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],

];

#[derive(Debug, PartialEq)]
pub struct Matrix44(MatrixArray);

// Overload for matrix multiplication
impl std::ops::Mul for Matrix44 {
    type Output = Matrix44;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m: MatrixArray = ZERO_MATRIX;

        for i in 0..4 { 
            for j in 0..4 {
                m[i][j] = self.0[i][0] * rhs.0[0][j] +
                          self.0[i][1] * rhs.0[1][j] +
                          self.0[i][2] * rhs.0[2][j] +
                          self.0[i][3] * rhs.0[3][j];
            }
        }

        Matrix44::new(m)
    }
}

impl Matrix44 {
    // Construct matrix from matrix array
    pub fn new(matrix_array: MatrixArray) -> Self {
        Matrix44(matrix_array)
    }

    // Return new identity matrix
    pub fn identity() -> Self {
        Matrix44 ([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    // Return the transpose of the current matrix
    pub fn transpose(&self) -> Self {
        let mut m: MatrixArray = ZERO_MATRIX;
        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = self.0[j][i];
            }
        }
        Matrix44::new(m)
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        let v2 = Vec3::new(4.0, 2.0, 5.0);

        let dot = v1.dot(&v2);
        assert_eq!(dot, 45.0);
    }
    
    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let cross = v1.cross(&v2);
        assert_eq!(cross, Vec3::new(-3.0, 6.0, -3.0));
    }
    
    #[test]
    fn test_len() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.len(), 3.741657387);
    }

    #[test]
    fn test_normalise() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v.normalise();

        assert_eq!(v, Vec3::new(0.26726124, 0.5345225, 0.8017837));
    }

    #[test]
    fn test_mult_matrix() {
        let vec = Vec3::new(3.0, 2.0, 4.0);

        let transformation = Matrix44::new([
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        let transformed_vec = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(vec.mult_matrix(&transformation), transformed_vec);
    }

    #[test]
    fn test_homogeneneous_mult_matrix() {
        let vec = Vec3::new(3.0, 2.0, 4.0);

        let transformation = Matrix44::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [10.0, 3.0, 5.0, 1.0],
        ]);

        let transformed_vec = Vec3::new(13.0, 5.0, 9.0);
        assert_eq!(vec.homogeneous_mult_matrix(&transformation), transformed_vec);
    }
}


#[cfg(test)]
mod matrix44_tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix44::new([
            [1.0, 0.0, 3.0, 4.0],
            [5.0, 2.0, 1.0, 2.0],
            [2.0, 1.0, 5.0, 6.0],
            [1.0, 2.0, 0.0, 4.0],
        ]);

        let b = a.transpose();

        let c = Matrix44::new([
            [26.0, 16.0, 41.0, 17.0],
            [16.0, 34.0, 29.0, 17.0],
            [41.0, 29.0, 66.0, 28.0],
            [17.0, 17.0, 28.0, 21.0],
        ]);

        assert_eq!(a * b, c);
    }
}

// Keeping generic implementation for this commented in the file cause it took me some work
// It's not really useful though
// use std::ops::{Add, Sub, Mul};
// trait Num: Copy + Mul<Output = Self> + Add<Output = Self> + Sub<Output = Self> + Into + std::convert::From {}

// pub struct Vec3<f32: Num> {
//    pub x: f32,
//    pub y: f32,
//    pub z: f32, 
// }

// impl<f32: Num> Vec3 {
//     pub fn new(x: f32, y: f32, z: f32) -> Self {
//         Vec3 {
//             x: x,
//             y: y,
//             z: z,
//         }
//     }
    
//     pub fn splat(d: f32) -> Self {
//         Vec3 {
//             x: d.clone(),
//             y: d.clone(),
//             z: d.clone(),
//         }
//     }

//     pub fn dot(&self, v: &Vec3) -> f32 {
//         self.x * v.x + self.y * v.y + self.z * v.z
//     }

//     pub fn cross(&self, v: &Vec3) -> Vec3 {
//        Vec3::new(
//             self.y * v.z - self.z * v.y,
//             self.z * v.x - self.x * v.z,
//             self.x * v.y - self.y * v.x,
//        ) 
//     }

//     pub fn len(&self) -> f32 {
//        f32::sqrt((self.x * self.x + self.y * self.y + self.z * self.z).into()) 
//     }

//     pub fn normalise(&mut self) {
//         let normalisation_constant: f32 = (1.0 / self.len()).into();
        
//         self.x = self.x * normalisation_constant;
//         self.y = self.y * normalisation_constant;
//         self.z = self.z * normalisation_constant;
//     }
// }
