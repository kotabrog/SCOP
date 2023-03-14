pub mod sample;

use crate::img_loader::Image;


pub struct Texture {
    id: gl::types::GLuint,
    width: usize,
    height: usize,
    data: Vec<f32>,
}

impl Texture {
    pub fn from_bmp_file(path: &str) -> Result<Self, String> {
        let img = Image::new(path)?;
        let width = img.get_width();
        let height = img.get_height();
        let data = img.get_data().clone();
        Ok(Self { id: 0, width, height, data })
    }

    pub fn set_texture(&mut self) {
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::BindBuffer(gl::TEXTURE_2D, self.id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                self.width as i32,
                self.height as i32,
                0,
                gl::RGB,
                gl::FLOAT,
                self.data.as_ptr() as *const _,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }
    }
}