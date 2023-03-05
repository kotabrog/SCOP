pub mod color_sample;
pub mod sample;

use std::ffi::CString;
use crate::mat::{Vec3d, Vec2d};


pub struct Model {
    vertices: Vec<Vec3d>,
    colors: Vec<Vec3d>,
    uv: Vec<Vec2d>,
    indices: Vec<gl::types::GLuint>,
    vbo: gl::types::GLuint,
    cbo: gl::types::GLuint,
    tbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
    max_size: f32,
    texture_on: TextureSwitch,
}

struct TextureSwitch {
    id: i32,
    texture_on: bool,
}

impl Model {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            colors: Vec::new(),
            uv: Vec::new(),
            indices: Vec::new(),
            vbo: 0,
            cbo: 0,
            tbo: 0,
            ebo: 0,
            max_size: 0.0,
            texture_on: TextureSwitch::new()
        }
    }

    pub fn set_texture(&mut self, program_id: gl::types::GLuint) -> Result<(), String> {
        self.texture_on.set_program(program_id)
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
        unsafe {
            gl::GenBuffers(1, &mut self.tbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.uv.len() * std::mem::size_of::<Vec2d>()) as gl::types::GLsizeiptr,
                self.uv.as_ptr() as *const gl::types::GLvoid,
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
        unsafe {
            gl::EnableVertexAttribArray(2);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null(),
            );
        }
    }

    pub fn switch_texture(&mut self) {
        self.texture_on.switch();
    }

    pub fn draw(&self) {
        self.texture_on.set();
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
}

impl TextureSwitch {
    pub fn new() -> Self {
        Self { id: 0, texture_on: false }
    }

    pub fn set_program(&mut self, program_id: gl::types::GLuint) -> Result<(), String> {
        let name = CString::new("IsTexture").map_err(|_| "error: GetUniformLocation".to_string())?;
        let id = unsafe {gl::GetUniformLocation(program_id, name.as_ptr()) };
        self.id = id;
        Ok(())
    }

    pub fn switch(&mut self) {
        self.texture_on = !self.texture_on;
    }

    pub fn set(&self) {
        unsafe {
            gl::Uniform1i(self.id, self.texture_on as i32);
        }
    }
}
