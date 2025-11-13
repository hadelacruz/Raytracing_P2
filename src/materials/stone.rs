use crate::math::Vec3;
use super::{Material, MaterialType};

pub fn create() -> Material {
    Material {
        material_type: MaterialType::Stone,
        albedo: Vec3::new(1.0, 1.0, 1.0),
        reflectivity: 0.1,
        transparency: 0.0,
        refractive_index: 1.0,
        emissive: Vec3::zero(),
    }
}
