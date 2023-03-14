#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    elem: [[f32; 4]; 4],
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vec3d {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Vec2d {
    pub d0: f32,
    pub d1: f32,
}

#[allow(dead_code)]
pub enum Axis {
    X,
    Y,
    Z
}

impl Matrix {
    pub fn new(vec: [[f32; 4]; 4]) -> Self {
        Self { elem: vec }
    }

    #[allow(dead_code)]
    pub fn inner(&mut self) -> &[[f32; 4]; 4] {
        &self.elem
    }

    pub fn make_identity_matrix() -> Self {
        Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn make_scale_matrix(value: f32) -> Self {
        Matrix::new([
            [value, 0.0, 0.0, 0.0],
            [0.0, value, 0.0, 0.0],
            [0.0, 0.0, value, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[allow(dead_code)]
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

    pub fn make_rotate_matrix(vec: &Vec3d, theta: f32) -> Self {
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
            [0.0, 0.0, (far + near) / (far - near), -1.0],
            [0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0],
        ])
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        let mut matrix = Matrix::new([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        for i in 0..4 {
            for j in 0..4 {
                matrix.elem[i][j] = self.mul_i_j(rhs, i, j)
            }
        }
        matrix
    }

    fn mul_i_j(&self, rhs: &Self, i: usize, j: usize) -> f32 {
        let mut ret = 0.0;
        for k in 0..4 {
            ret += self.elem[i][k] * rhs.elem[k][j];
        }
        ret
    }

    #[allow(dead_code)]
    pub fn normalize_part(&self) -> Self {
        let mut matrix = self.clone();
        for i in 0..3 {
            let mut norm = 0.0;
            for j in 0..3 {
                norm += self.elem[i][j] * self.elem[i][j];
            }
            norm = norm.sqrt();
            for j in 0..3 {
                matrix.elem[i][j] = self.elem[i][j] / norm;
            }
        }
        matrix
    }

    pub fn orthonormalization(&self) -> Self {
        let x1 = Vec3d::new(self.elem[0][0], self.elem[0][1], self.elem[0][2]);
        let a2 = Vec3d::new(self.elem[1][0], self.elem[1][1], self.elem[1][2]);
        let a3 = Vec3d::new(self.elem[2][0], self.elem[2][1], self.elem[2][2]);
        let x2 = a2.minus(&x1.mul(a2.inner_product(&x1) / x1.inner_product(&x1)));
        let x3 = a3.minus(
            &x1.mul(a3.inner_product(&x1) / x1.inner_product(&x1)).add(
            &x2.mul(a3.inner_product(&x2) / x2.inner_product(&x2))
        ));
        let x1 = x1.normalize();
        let x2 = x2.normalize();
        let x3 = x3.normalize();
        Self::new([
            [x1.d0, x1.d1, x1.d2, 0.0],
            [x2.d0, x2.d1, x2.d2, 0.0],
            [x3.d0, x3.d1, x3.d2, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Vec3d {
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

    pub fn inner_product(&self, rhs: &Self) -> f32 {
        self.d0 * rhs.d0 + self.d1 * rhs.d1 + self.d2 * rhs.d2
    }

    pub fn add(&self, rhs: &Self) -> Self {
        (self.d0 + rhs.d0, self.d1 + rhs.d1, self.d2 + rhs.d2).into()
    }

    pub fn minus(&self, rhs: &Self) -> Self {
        (self.d0 - rhs.d0, self.d1 - rhs.d1, self.d2 - rhs.d2).into()
    }

    pub fn mul(&self, rhs: f32) -> Self {
        (self.d0 * rhs, self.d1 * rhs, self.d2 * rhs).into()
    }
}

impl From<(f32, f32, f32)> for Vec3d {
    fn from(other: (f32, f32, f32)) -> Self {
        Vec3d::new(other.0, other.1, other.2)
    }
}

impl Vec2d {
    pub fn new(d0: f32, d1: f32) -> Self {
        Self { d0, d1 }
    }
}

impl From<(f32, f32)> for Vec2d {
    fn from(other: (f32, f32)) -> Self {
        Vec2d::new(other.0, other.1)
    }
}
