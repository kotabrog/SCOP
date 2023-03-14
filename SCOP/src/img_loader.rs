use std::fs::File;
use std::io::Read;
use std::ffi::c_void;


pub struct Image {
    data: Vec<f32>,
    width: usize,
    height: usize,
}

enum Mode {
    Gray,
    RGB,
    RGBA,
}

impl Image {
    pub fn new(path: &str) -> Result<Self, String> {
        let data = Self::load_file(path)?;
        let (width, height, mode) = Self::load_bmp_header(&data)
            .map_err(|e| format!("{}: {}", e, path))?;
        Ok(Image {
            data: Self::load_bmp_data(&data, width, height, mode),
            width,
            height,
        })
    }

    pub fn get_data(&self) -> &Vec<f32> {
        &self.data
    }

    #[allow(dead_code)]
    pub fn get_ptr(&self) -> *const c_void {
        self.data.as_ptr() as *const c_void
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn load_file(path: &str) -> Result<Vec<u8>, String> {
        let mut file = File::open(path)
            .map_err(|e| format!("error: {}: {}", path, e))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| format!("error: {}", e))?;
        Ok(buf)
    }

    fn load_bmp_header(data: &Vec<u8>) -> Result<(usize, usize, Mode), String> {
        // There should be 54bite in the header.
        if data.len() < 54 || data[0] != 'B' as u8 || data[1] != 'M' as u8 {
            return Err(format!("error: not bmp"))
        }

        let header_size = data[0x0A] as usize;
        let image_size = data[0x22] as usize;
        let width = data[0x12] as usize;
        let height = data[0x16] as usize;
        if header_size != 0 && header_size != 54 {
            return Err(format!("error: not supported bmp: header_size != 54"))
        }
        if image_size != 0 && image_size != width * height * 3 {
            return Err(format!("error: not supported bmp: image_size != width * height * 3"))
        }
        if !Self::is_pow2(width * height) {
            return Err(format!("error: not supported bmp: size != 2^n"))
        }
        let mode = if data.len() - 54 == width * height {
            Mode::Gray
        } else if data.len() - 54 == width * height * 3 {
            Mode::RGB
        } else if data.len() - 54 == width * height * 4 {
            Mode::RGBA
        } else {
            return Err(format!("error: not supported bmp: data size"))
        };
        Ok((width as usize, height as usize, mode))
    }

    fn is_pow2(value: usize) -> bool {
        let mut value = value;
        if value % 2 != 0 {
            return false
        }
        while value > 1 {
            value /= 2;
            if value != 1 && value % 2 != 0 {
                return false
            }
        }
        true
    }

    fn load_bmp_data(data: &Vec<u8>, width: usize, height: usize, mode: Mode) -> Vec<f32> {
        let mut img_data = vec![0.0; 3 * width * height];
        for i in 0..height {
            for j in 0..width {
                let img_offset = i * width * 3 + j * 3;
                match mode {
                    Mode::Gray => {
                        let value = data[i * width + j + 54] as f32 / 256.0;
                        img_data[img_offset + 0] = value;
                        img_data[img_offset + 1] = value;
                        img_data[img_offset + 2] = value;
                    },
                    _ => {
                        let data_offset = match mode {
                            Mode::RGB => i * width * 3 + j * 4 + 54,
                            Mode::RGBA => i * width * 4 + j * 4 + 54,
                            _ => 0,
                        };
                        img_data[img_offset + 0] = data[data_offset + 2] as f32 / 256.0;
                        img_data[img_offset + 1] = data[data_offset + 1] as f32 / 256.0;
                        img_data[img_offset + 2] = data[data_offset + 0] as f32 / 256.0;
                    }
                }
            }
        }
        img_data
    }
}
