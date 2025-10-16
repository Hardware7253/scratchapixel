//This file contains definitions and implementations for vector and matrix types 

use crate::num::Num;

#[derive(Debug, PartialEq)]
pub struct Vec2<T: Num> {
   pub x: T,
   pub y: T,
}

impl<T: Num> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 {
            x: x,
            y: y,
        }
    }
    
    pub fn splat(d: T) -> Self {
        Vec2 {
            x: d.clone(),
            y: d.clone(),
        }
    }

    pub fn from_vec3(vec: &Vec3<T>) -> Self {
        Vec2 {
            x: vec.x.clone(),
            y: vec.y.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Vec3<T: Num> {
   pub x: T,
   pub y: T,
   pub z: T, 
}

impl<T: Num> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
    
    pub fn splat(d: T) -> Self {
        Vec3 {
            x: d.clone(),
            y: d.clone(),
            z: d.clone(),
        }
    }

    pub fn from_vec2(vec: &Vec2<T>, z: T) -> Self {
        Vec3 {
            x: vec.x.clone(),
            y: vec.y.clone(),
            z: z.clone(),
        }
    }

    // Does vector dot product with another vector
    pub fn dot(&self, v: &Vec3<T>) -> T {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    // Does vector cross product with another vector
    pub fn cross(&self, v: &Vec3<T>) -> Vec3<T> {
       Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
       ) 
    }

    // Returns vector 2 norm
    pub fn len(&self) -> T where T: From<f32> + Into<f32> {
       f32::sqrt((self.x * self.x + self.y * self.y + self.z * self.z).into()).into()
    }

    // Makes vector length 1
    pub fn normalise(&mut self) where T: From<f32> + Into<f32> {
        let normalisation_constant: T = (1.0 / self.len().into()).into();
        
        self.x = self.x * normalisation_constant;
        self.y = self.y * normalisation_constant;
        self.z = self.z * normalisation_constant;
    }

    // Multiply matrices [1x3] x [3x3] = [1x3]
    // The Matrix44 is treated as a 3x3 matrix to perform this multiplication
    pub fn mult_matrix(&self, matrix: &Matrix44) -> Vec3<T> where T: From<f32> + Into<f32> {
        let mut vec_array: [f32; 3] = [0.0; 3];
        for i in 0..3 {
            vec_array[i] = self.x.into() * matrix.0[0][i] +
                           self.y.into() * matrix.0[1][i] +
                           self.z.into() * matrix.0[2][i];
        }

        Vec3::new(vec_array[0].into(), vec_array[1].into(), vec_array[2].into())
    }

    // Multiply matrices [1x4] x [4x4] = [1x4]
    // The homogenous coordinate of the input vector is implied to be one
    // The homogeneous output coordinates are normalised so a Vec3 can be returned
    pub fn homogeneous_mult_matrix(&self, matrix: &Matrix44) -> Vec3<T> where T: From<f32> + Into<f32> {
        let mut vec_array: [f32; 4] = [0.0; 4];
        for i in 0..4 {
            vec_array[i] = self.x.into() * matrix.0[0][i] +
                           self.y.into() * matrix.0[1][i] +
                           self.z.into() * matrix.0[2][i] +
                        /* implicit 1 * */ matrix.0[3][i];
        }

        // Convert homogeneous coordinates back to cartesian
        vec_array[0] /= vec_array[3];
        vec_array[1] /= vec_array[3];
        vec_array[2] /= vec_array[3];

        Vec3::new(vec_array[0].into(), vec_array[1].into(), vec_array[2].into())
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



