use crate::math::Vec3;
use super::{Material, MaterialType};

pub fn create() -> Material {
    Material {
        material_type: MaterialType::Sun,
        albedo: Vec3::new(1.0, 0.9, 0.7),
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        emissive: Vec3::new(5.0, 4.0, 2.0),
    }
}
