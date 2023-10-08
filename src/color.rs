use glam::Vec3;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
        }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }

    // For writing to file
    pub fn to_str(&self) -> String {
        format!("{} {} {}\n", (255.999 * self.r) as i32, (255.999 * self.g) as i32, (255.999 * self.b) as i32)
    }
}

