use crate::math::Vec3;
use super::{Material, MaterialType};

pub fn create() -> Material {
    Material {
        material_type: MaterialType::FireParticle,
        albedo: Vec3::new(1.0, 0.6, 0.1),
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        emissive: Vec3::new(3.0, 1.5, 0.3),
    }
}

pub fn get_animated_emissive(time: f32, position: &Vec3) -> Vec3 {
    let flicker = (time * 10.0 + position.x + position.y + position.z).sin() * 0.3 + 0.7;
    Vec3::new(3.0 * flicker, 1.5 * flicker, 0.3 * flicker)
}
