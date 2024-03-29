use super::Model;
use crate::mat::{Vec3d, Vec2d};

impl Model {
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
        self.set_index_set(3);
        self.index_count = vec![3, 3]
    }

    #[allow(dead_code)]
    pub fn set_rect_uv_sample(&mut self) {
        self.vertices = vec![
            Vec3d::new(-0.7, -0.7, 0.0),
            Vec3d::new(0.7, -0.7, 0.0),
            Vec3d::new(0.7, 0.7, 0.0),
            Vec3d::new(-0.7, 0.7, 0.0),
        ];
        self.uv = vec![
            Vec2d::new(0.0, 0.0),
            Vec2d::new(1.0, 0.0),
            Vec2d::new(1.0, 1.0),
            Vec2d::new(0.0, 1.0),
        ];
        self.indices = vec![
            0, 1, 2, 0, 3, 2
        ];
        self.max_size = 1.0;
        self.set_index_set(3);
        self.index_count = vec![3, 3]
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

    #[allow(dead_code)]
    pub fn set_cube_sample_uv(&mut self) {
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
        self.uv = vec![
            Vec2d::new(0.000059, 1.0-0.000004),
            Vec2d::new(0.000103, 1.0-0.336048),
            Vec2d::new(0.335973, 1.0-0.335903),
            Vec2d::new(1.000023, 1.0-0.000013),
            Vec2d::new(0.667979, 1.0-0.335851),
            Vec2d::new(0.999958, 1.0-0.336064),
            Vec2d::new(0.667979, 1.0-0.335851),
            Vec2d::new(0.336024, 1.0-0.671877),
            Vec2d::new(0.667969, 1.0-0.671889),
            Vec2d::new(1.000023, 1.0-0.000013),
            Vec2d::new(0.668104, 1.0-0.000013),
            Vec2d::new(0.667979, 1.0-0.335851),
            Vec2d::new(0.000059, 1.0-0.000004),
            Vec2d::new(0.335973, 1.0-0.335903),
            Vec2d::new(0.336098, 1.0-0.000071),
            Vec2d::new(0.667979, 1.0-0.335851),
            Vec2d::new(0.335973, 1.0-0.335903),
            Vec2d::new(0.336024, 1.0-0.671877),
            Vec2d::new(1.000004, 1.0-0.671847),
            Vec2d::new(0.999958, 1.0-0.336064),
            Vec2d::new(0.667979, 1.0-0.335851),
            Vec2d::new(0.668104, 1.0-0.000013),
            Vec2d::new(0.335973, 1.0-0.335903),
            Vec2d::new(0.667979, 1.0-0.335851),
            Vec2d::new(0.335973, 1.0-0.335903),
            Vec2d::new(0.668104, 1.0-0.000013),
            Vec2d::new(0.336098, 1.0-0.000071),
            Vec2d::new(0.000103, 1.0-0.336048),
            Vec2d::new(0.000004, 1.0-0.671870),
            Vec2d::new(0.336024, 1.0-0.671877),
            Vec2d::new(0.000103, 1.0-0.336048),
            Vec2d::new(0.336024, 1.0-0.671877),
            Vec2d::new(0.335973, 1.0-0.335903),
            Vec2d::new(0.667969, 1.0-0.671889),
            Vec2d::new(1.000004, 1.0-0.671847),
            Vec2d::new(0.667979, 1.0-0.335851)
        ];
        self.max_size = 1.0;
    }
}