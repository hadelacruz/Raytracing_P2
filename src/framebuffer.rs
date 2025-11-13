use crate::math::Vec3;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vec3>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![Vec3::zero(); width * height],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(Vec3::zero());
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    fn vec3_to_u32(&self, color: Vec3) -> u32 {
        let r = (color.x.clamp(0.0, 1.0).sqrt() * 255.0) as u32;
        let g = (color.y.clamp(0.0, 1.0).sqrt() * 255.0) as u32;
        let b = (color.z.clamp(0.0, 1.0).sqrt() * 255.0) as u32;
        
        (r << 16) | (g << 8) | b
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.buffer
            .iter()
            .map(|color| self.vec3_to_u32(*color))
            .collect()
    }
}
