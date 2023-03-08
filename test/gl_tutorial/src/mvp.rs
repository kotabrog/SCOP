use std::ffi::CString;

use crate::mat::{Matrix, Vec3d};


const MAX_SCALE: f32 = 1000.0;
const DEFAULT_TRANSLATION_Z: f32 = 2.0;
const PROJECTION_FOV: f32 = std::f32::consts::FRAC_PI_4;
const PROJECTION_NEAR: f32 = -1.0;
const PROJECTION_FAR: f32 = 20.0;
const TRANSLATION_SPEED: f32 = 0.5;
const TRANSLATION_WHEEL_SPEED: f32 = 0.5;
const ROTATION_SPEED: f32 = 0.5;


#[derive(Debug)]
pub struct MVP {
    center_matrix: MVPMatrix,
    scale_matrix: MVPMatrix,
    rotation_matrix: MVPMatrix,
    translation_matrix: MVPMatrix,
    projection_matrix: MVPMatrix,
    translation_vec: Vec3d,
}

#[derive(Debug)]
pub struct MVPMatrix {
    name: CString,
    id: i32,
    matrix: Matrix,
}

impl MVP {
    pub fn new(program_id: gl::types::GLuint,
        scale_max_size: f32,
        vertices: &Vec<Vec3d>,
        window_size: (u32, u32)
    ) -> Result<Self, String> {
        let center_matrix = Self::make_center_matrix(vertices);
        let scale_matrix = Self::make_scale_matrix(scale_max_size);
        let rotation_matrix = Matrix::make_identity_matrix();
        let translation_vec = Vec3d::new(0.0, 0.0, DEFAULT_TRANSLATION_Z);
        let translation_matrix = Self::make_translation_matrix(&translation_vec);
        let projection_matrix = Self::make_projection_matrix(window_size);

        Ok(Self {
            center_matrix: MVPMatrix::new(
                program_id, "Center", center_matrix
            )?,
            scale_matrix: MVPMatrix::new(
                program_id, "Scale", scale_matrix
            )?,
            rotation_matrix: MVPMatrix::new(
                program_id, "Rotation", rotation_matrix
            )?,
            translation_matrix: MVPMatrix::new(
                program_id, "Translation", translation_matrix
            )?,
            projection_matrix: MVPMatrix::new(
                program_id, "Projection", projection_matrix
            )?,
            translation_vec
        })
    }

    fn make_center_matrix(vertices: &Vec<Vec3d>) -> Matrix {
        let mut vec = Vec3d::new(0.0, 0.0, 0.0);
        for v in vertices {
            vec = vec.add(v);
        }
        vec = vec.mul(- 1.0 / vertices.len() as f32);
        println!("{:?}", vec);
        return Self::make_translation_matrix(&vec)
    }

    fn make_scale_matrix(scale_max_size: f32) -> Matrix {
        Matrix::make_scale_matrix((1.0 / scale_max_size).min(MAX_SCALE))
    }

    fn make_translation_matrix(translation_vec: &Vec3d) -> Matrix {
        Matrix::make_translation_matrix(
            translation_vec.d0,
            translation_vec.d1,
            translation_vec.d2
        )
    }

    fn update_translation_matrix(&mut self) {
        self.translation_matrix.matrix_update(
            Self::make_translation_matrix(&self.translation_vec)
        )
    }

    fn make_projection_matrix(window_size: (u32, u32)) -> Matrix {
        Matrix::make_perspective_projection_matrix(
            PROJECTION_FOV,
            window_size.0 as f32 / window_size.1 as f32,
            PROJECTION_NEAR,
            PROJECTION_FAR
        )
    }

    pub fn update_translation_mouse(&mut self, x: f32, y: f32, diff_time: f32) {
        self.translation_vec.d0 += x * TRANSLATION_SPEED * diff_time / 1000.0;
        self.translation_vec.d1 -= y * TRANSLATION_SPEED * diff_time / 1000.0;
        self.update_translation_matrix();
    }

    pub fn update_translation_wheel(&mut self, z: f32) {
        self.translation_vec.d2 += z * TRANSLATION_WHEEL_SPEED;
        self.update_translation_matrix();
    }

    pub fn update_rotate_matrix(&mut self, x: i32, y: i32, diff_time: f32) {
        let direction_vec = if y == 0 {
            let v = if x > 0 {1} else {-1} as f32;
            Vec3d::new(0.0, v, 0.0)
        } else if x == 0 {
            let v = if y > 0 {1} else {-1} as f32;
            Vec3d::new(v, 0.0, 0.0)
        } else {
            Vec3d::new(y as f32, x as f32, 0.0).normalize()
        };
        let rotation_matrix =
            Matrix::make_rotate_matrix(
                &direction_vec,
                std::f32::consts::PI * ROTATION_SPEED * diff_time / 1000.0
            ).mul(&self.rotation_matrix.matrix)
            .orthonormalization();
        self.rotation_matrix.matrix_update(rotation_matrix);
    }

    pub fn update_z_rotate_matrix(&mut self, x: i32, diff_time: f32) {
        let direction_vec = Vec3d::new(0.0, 0.0, 1.0);
        let rotation_matrix =
            Matrix::make_rotate_matrix(
                &direction_vec,
                x as f32 * std::f32::consts::PI * ROTATION_SPEED * diff_time / 1000.0
            ).mul(&self.rotation_matrix.matrix)
            .orthonormalization();
        self.rotation_matrix.matrix_update(rotation_matrix);
    }

    pub fn set(&self) {
        self.center_matrix.set();
        self.scale_matrix.set();
        self.rotation_matrix.set();
        self.translation_matrix.set();
        self.projection_matrix.set();
    }
}

impl MVPMatrix {
    pub fn new(program_id: gl::types::GLuint, name: &str, matrix: Matrix) -> Result<Self, String> {
        let name = CString::new(name).map_err(|_| "error: GetUniformLocation".to_string())?;
        let id = unsafe {gl::GetUniformLocation(program_id, name.as_ptr()) };
        Ok(Self { name, id, matrix })
    }

    pub fn matrix(&self) -> &Matrix {
        &self.matrix
    }

    pub fn name(&self) -> &CString {
        &self.name
    }

    pub fn matrix_update(&mut self, matrix: Matrix) {
        self.matrix = matrix;
    }

    pub fn set(&self) {
        unsafe {
            let ptr = std::mem::transmute(&self.matrix);
            gl::UniformMatrix4fv(self.id, 1, gl::TRUE, ptr);
        }
    }
}
