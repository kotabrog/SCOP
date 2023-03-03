#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    elem: [[f32; 4]; 4],
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Point {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

pub enum Axis {
    X,
    Y,
    Z
}

impl Matrix {
    pub fn new(vec: [[f32; 4]; 4]) -> Self {
        Self { elem: vec }
    }

    pub fn inner(&mut self) -> &[[f32; 4]; 4] {
        &self.elem
    }

    pub fn make_rotate_one_axis_matrix(axis: Axis, theta: f32) -> Self {
        match axis {
            Axis::X => Matrix::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, theta.cos(), -theta.sin(), 0.0],
                [0.0, theta.sin(), theta.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            Axis::Y => Matrix::new([
                [theta.cos(), 0.0, -theta.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [theta.sin(), 0.0, theta.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            Axis::Z => Matrix::new([
                [theta.cos(), -theta.sin(), 0.0, 0.0],
                [theta.sin(), theta.cos(), 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
        }
    }

    pub fn make_rotate_matrix(vec: &Point, theta: f32) -> Self {
        // vec is the normalized direction vector
        let c = theta.cos();
        let s = theta.sin();
        let mc = 1.0 - c;
        let x = vec.d0;
        let y = vec.d1;
        let z = vec.d2;
        Matrix::new([
            [x * x * mc + c, x * y * mc - z * s, x * z * mc + y * s, 0.0],
            [x * y * mc + z * s, y * y * mc + c, y * z * mc - x * s, 0.0],
            [x * z * mc + y * s, z * y * mc + x * s, z * z * mc + c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn make_translation_matrix(x: f32, y: f32, z: f32) -> Self {
        Matrix::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn make_perspective_projection_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let frac_tan = 1.0 / fov.tan();
        Matrix::new([
            [frac_tan, 0.0, 0.0, 0.0],
            [0.0, frac_tan * aspect, 0.0, 0.0],
            [0.0, 0.0, (far + near) / (far - near), 0.0],
            [0.0, 0.0, -(2.0 * far * near) / (far - near), 1.0],
        ])
    }
}

impl Point {
    pub fn new(d0: f32, d1: f32, d2: f32) -> Self {
        Self { d0, d1, d2 }
    }

    pub fn length(&self) -> f32 {
        (self.d0 * self.d0 + self.d1 * self.d1 + self.d2 * self.d2).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();

        Self::new(self.d0 / length, self.d1 / length, self.d2 / length)
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from(other: (f32, f32, f32)) -> Self {
        Point::new(other.0, other.1, other.2)
    }
}
