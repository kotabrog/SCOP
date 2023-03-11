pub mod color_sample;
pub mod sample;

use std::ffi::CString;
use crate::mat::{Vec3d, Vec2d};


pub struct Model {
    vertices: Vec<Vec3d>,
    colors: Vec<Vec3d>,
    uv: Vec<Vec2d>,
    indices: Vec<gl::types::GLuint>,
    index_set: usize,
    index_count: Vec<usize>,
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
            index_set: 0,
            index_count: Vec::new(),
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

    pub fn get_vertices(&self) -> &Vec<Vec3d> {
        &self.vertices
    }

    pub fn get_index_set(&self) -> usize {
        self.index_set
    }

    pub fn set_index_set(&mut self, index_set: usize) {
        self.index_set = index_set
    }

    pub fn push_index_count(&mut self, count: usize) {
        self.index_count.push(count);
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

    pub fn resolve_duplicate_indices(&mut self) {
        if self.indices.is_empty() {
            return;
        }
        let mut vertices = Vec::new();
        for index in &self.indices {
            vertices.push(self.vertices[*index as usize])
        }
        self.indices = (0..vertices.len() as gl::types::GLuint).collect();
        self.vertices = vertices;
    }

    fn set_uv_to_non_indices(&mut self) {
        for i in 0..self.vertices.len() {
            if i % 4 == 0 {
                self.uv.push(Vec2d::new(0.0, 0.0));
            } else if i % 4 == 1 {
                self.uv.push(Vec2d::new(1.0, 0.0));
            } else if i % 4 == 2 {
                self.uv.push(Vec2d::new(0.0, 1.0));
            } else {
                self.uv.push(Vec2d::new(1.0, 1.0));
            }
        }
    }

    fn set_uv_to_has_indices(&mut self) {
        for i in 0..self.vertices.len() {
            if i % 4 == 0 {
                self.uv.push(Vec2d::new(0.0, 0.0));
            } else if i % 4 == 1 {
                self.uv.push(Vec2d::new(1.0, 0.0));
            } else if i % 4 == 2 {
                self.uv.push(Vec2d::new(0.0, 1.0));
            } else {
                self.uv.push(Vec2d::new(1.0, 1.0));
            }
        }
    }

    pub fn set_uv(&mut self) {
        if !self.uv.is_empty() {
            return;
        } else if self.indices.is_empty() {
            self.set_uv_to_non_indices();
        } else {
            self.set_uv_to_has_indices();
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

    pub fn draw(&self) -> Result<(), String>{
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
            match self.index_set {
                3 => {
                    self.draw_one_elements(
                        0,
                        self.indices.len() as i32,
                        3
                    )?;
                },
                1 => {
                    let mut index = 0;
                    let mut count_index = 0;
                    let mut index_set = 0;
                    let mut start_index = 0;
                    while index < self.indices.len() &&
                            count_index < self.index_count.len() {
                        if index == start_index {
                            index_set = self.index_count[count_index];
                            index += index_set;
                            count_index += 1;
                        } else if index_set != 3 || index_set != self.index_count[count_index] {
                            self.draw_one_elements(
                                start_index,
                                (index - start_index) as i32,
                                index_set as i32
                            )?;
                            start_index = index;
                        } else {
                            index += index_set;
                            count_index += 1;
                        }
                    }
                    if index != start_index {
                        let count = if index != self.indices.len() {
                            (index - start_index).saturating_sub(1)
                        } else {
                            index - start_index
                        };
                        if count > 0 {
                            self.draw_one_elements(
                                start_index,
                                count as i32,
                                index_set as i32
                            )?;
                        }
                    }
                },
                _ => return Err(format!("error: index set is 3 or 4"))
            }
        }
        Ok(())
    }

    fn draw_one_elements(&self, index: usize, count: i32, index_set: i32) -> Result<(), String>{
        let mode = match index_set {
            1 | 2 => return Err(format!("error: unsupported index set 1 or 2")),
            3 => gl::TRIANGLES,
            _ => gl::TRIANGLE_FAN,
        };
        unsafe {
            gl::DrawElements(
                mode,
                count,
                gl::UNSIGNED_INT,
                (index * std::mem::size_of::<gl::types::GLuint>()) as *const std::os::raw::c_void
            );
        }
        Ok(())
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
