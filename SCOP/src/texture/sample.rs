use super::Texture;

impl Texture {
    pub fn red_gradation() -> Self {
        let width = 64;
        let height = 64;
        let mut data = vec![0.0; width * height * 3];
        let size = (width * height) as f32;
        for i in 0..height {
            for j in 0..width {
                let r = (i * j) as f32 / size / 2.0 + 0.5;
                data[i * width * 3 + j * 3 + 0] = r;
                data[i * width * 3 + j * 3 + 1] = 0.0;
                data[i * width * 3 + j * 3 + 2] = 0.0;
            }
        }
        Self { id: 0, width, height, data }
    }
}
