use crate::mat::{Vec3d, Matrix};


pub struct Model {
    vertices: Vec<Vec3d>,
    colors: Vec<Vec3d>,
    indices: Vec<gl::types::GLuint>,
    vbo: gl::types::GLuint,
    cbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
    max_size: f32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            colors: Vec::new(),
            indices: Vec::new(),
            vbo: 0,
            cbo: 0,
            ebo: 0,
            max_size: 0.0,
        }
    }

    pub fn get_max_size(&self) -> f32 {
        self.max_size
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn push_vertices(&mut self, vec: Vec3d) {
        if vec.d0.abs() > self.max_size {
            self.max_size = vec.d0.abs();
        }
        if vec.d1.abs() > self.max_size {
            self.max_size = vec.d1.abs();
        }
        if vec.d2.abs() > self.max_size {
            self.max_size = vec.d2.abs();
        }
        self.vertices.push(vec);
    }

    pub fn push_indices(&mut self, value: gl::types::GLuint) {
        self.indices.push(value);
    }

    #[allow(dead_code)]
    pub fn set_colors_one(&mut self, color: f32) {
        self.colors = vec![Vec3d::new(color, color, color); self.vertices.len()];
    }

    #[allow(dead_code)]
    pub fn set_colors_gradation(&mut self) {
        let length = self.vertices.len();
        for i in 0..length {
            let value = i as f32 / length as f32;
            self.colors.push(Vec3d::new(value, value, value));
        }
    }

    #[allow(dead_code)]
    pub fn set_colors_gradation_colorful(&mut self) {
        let length = self.vertices.len();
        for i in 0..length {
            let value = i as f32 / length as f32;
            match i % 3 {
                0 => self.colors.push(Vec3d::new(value, 0.0, 0.0)),
                1 => self.colors.push(Vec3d::new(0.0, value, 0.0)),
                2 => self.colors.push(Vec3d::new(0.0, 0.0, value)),
                _ => {}
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_colors_grain(&mut self) {
        let length = self.vertices.len();
        for i in 0..length {
            match i % 3 {
                0 => self.colors.push(Vec3d::new(222.0 / 255.0, 184.0 / 255.0, 135.0 / 255.0)),
                1 => self.colors.push(Vec3d::new(210.0 / 255.0, 180.0 / 255.0, 140.0 / 255.0)),
                2 => self.colors.push(Vec3d::new(245.0 / 255.0, 222.0 / 255.0, 179.0 / 255.0)),
                _ => {}
            }
        }
    }

    pub fn set_buffers(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vec3d>()) as gl::types::GLsizeiptr,
                self.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
        unsafe {
            gl::GenBuffers(1, &mut self.cbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.cbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.colors.len() * std::mem::size_of::<Vec3d>()) as gl::types::GLsizeiptr,
                self.colors.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
        if !self.indices.is_empty() {
            unsafe {
                gl::GenBuffers(1, &mut self.ebo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (self.indices.len() * std::mem::size_of::<gl::types::GLuint>()) as gl::types::GLsizeiptr,
                    self.indices.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
            }
        }
    }

    pub fn config_buffer(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
        }
        unsafe {
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.cbo);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
        }
    }

    pub fn draw(&self) {
        if self.indices.is_empty() {
            unsafe {
                gl::DrawArrays(
                    gl::TRIANGLES,
                    0,
                    (self.vertices.len() * 3) as i32,
                );
            }
        } else {
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null()
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_rect_sample(&mut self) {
        self.vertices = vec![
            Vec3d::new(-0.7, -0.7, 0.0),
            Vec3d::new(0.7, -0.7, 0.0),
            Vec3d::new(0.7, 0.7, 0.0),
            Vec3d::new(-0.7, 0.7, 0.0),
        ];
        self.colors = vec![
            Vec3d::new(1.0, 1.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Vec3d::new(0.0, 0.0, 1.0),
            Vec3d::new(0.0, 1.0, 1.0),
        ];
        self.indices = vec![
            0, 1, 2, 0, 3, 2
        ];
        self.max_size = 1.0;
    }

    #[allow(dead_code)]
    pub fn set_cube_sample(&mut self) {
        self.vertices = vec![
            Vec3d::new(-0.5,-0.5,-0.5),
            Vec3d::new(-0.5,-0.5, 0.5),
            Vec3d::new(-0.5, 0.5, 0.5),
            Vec3d::new(0.5, 0.5,-0.5),
            Vec3d::new(-0.5,-0.5,-0.5),
            Vec3d::new(-0.5, 0.5,-0.5),
            Vec3d::new(0.5,-0.5, 0.5),
            Vec3d::new(-0.5,-0.5,-0.5),
            Vec3d::new(0.5,-0.5,-0.5),
            Vec3d::new(0.5, 0.5,-0.5),
            Vec3d::new(0.5,-0.5,-0.5),
            Vec3d::new(-0.5,-0.5,-0.5),
            Vec3d::new(-0.5,-0.5,-0.5),
            Vec3d::new(-0.5, 0.5, 0.5),
            Vec3d::new(-0.5, 0.5,-0.5),
            Vec3d::new(0.5,-0.5, 0.5),
            Vec3d::new(-0.5,-0.5, 0.5),
            Vec3d::new(-0.5,-0.5,-0.5),
            Vec3d::new(-0.5, 0.5, 0.5),
            Vec3d::new(-0.5,-0.5, 0.5),
            Vec3d::new(0.5,-0.5, 0.5),
            Vec3d::new(0.5, 0.5, 0.5),
            Vec3d::new(0.5,-0.5,-0.5),
            Vec3d::new(0.5, 0.5,-0.5),
            Vec3d::new(0.5,-0.5,-0.5),
            Vec3d::new(0.5, 0.5, 0.5),
            Vec3d::new(0.5,-0.5, 0.5),
            Vec3d::new(0.5, 0.5, 0.5),
            Vec3d::new(0.5, 0.5,-0.5),
            Vec3d::new(-0.5, 0.5,-0.5),
            Vec3d::new(0.5, 0.5, 0.5),
            Vec3d::new(-0.5, 0.5,-0.5),
            Vec3d::new(-0.5, 0.5, 0.5),
            Vec3d::new(0.5, 0.5, 0.5),
            Vec3d::new(-0.5, 0.5, 0.5),
            Vec3d::new(0.5,-0.5, 0.5)
        ];
        self.colors = vec![
            Vec3d::new(0.583,  0.771,  0.014),
            Vec3d::new(0.609,  0.115,  0.436),
            Vec3d::new(0.327,  0.483,  0.844),
            Vec3d::new(0.822,  0.569,  0.201),
            Vec3d::new(0.435,  0.602,  0.223),
            Vec3d::new(0.310,  0.747,  0.185),
            Vec3d::new(0.597,  0.770,  0.761),
            Vec3d::new(0.559,  0.436,  0.730),
            Vec3d::new(0.359,  0.583,  0.152),
            Vec3d::new(0.483,  0.596,  0.789),
            Vec3d::new(0.559,  0.861,  0.639),
            Vec3d::new(0.195,  0.548,  0.859),
            Vec3d::new(0.014,  0.184,  0.576),
            Vec3d::new(0.771,  0.328,  0.970),
            Vec3d::new(0.406,  0.615,  0.116),
            Vec3d::new(0.676,  0.977,  0.133),
            Vec3d::new(0.971,  0.572,  0.833),
            Vec3d::new(0.140,  0.616,  0.489),
            Vec3d::new(0.997,  0.513,  0.064),
            Vec3d::new(0.945,  0.719,  0.592),
            Vec3d::new(0.543,  0.021,  0.978),
            Vec3d::new(0.279,  0.317,  0.505),
            Vec3d::new(0.167,  0.620,  0.077),
            Vec3d::new(0.347,  0.857,  0.137),
            Vec3d::new(0.055,  0.953,  0.042),
            Vec3d::new(0.714,  0.505,  0.345),
            Vec3d::new(0.783,  0.290,  0.734),
            Vec3d::new(0.722,  0.645,  0.174),
            Vec3d::new(0.302,  0.455,  0.848),
            Vec3d::new(0.225,  0.587,  0.040),
            Vec3d::new(0.517,  0.713,  0.338),
            Vec3d::new(0.053,  0.959,  0.120),
            Vec3d::new(0.393,  0.621,  0.362),
            Vec3d::new(0.673,  0.211,  0.457),
            Vec3d::new(0.820,  0.883,  0.371),
            Vec3d::new(0.982,  0.099,  0.879)
        ];
        self.max_size = 1.0;
    }
}