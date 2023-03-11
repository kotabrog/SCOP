use super::Model;
use crate::mat::Vec3d;

impl Model {
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

    #[allow(dead_code)]
    pub fn set_colors_gray(&mut self, class: usize) {
        let length = if self.indices.is_empty() {
            self.vertices.len() / 3
        } else {
            self.index_count.len()
        };
        for i in 0..length {
            let color = ((i % class + 1) as f32) / (class + 1) as f32;
            let count = if self.indices.is_empty() {
                3
            } else {
                self.index_count[i]
            };
            for _ in 0..count {
                self.colors.push(Vec3d::new(color, color, color));
            }
        }
    }
}
