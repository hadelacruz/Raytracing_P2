use crate::math::Vec3;
use super::{Material, MaterialType, pixelated_pattern};

pub fn create() -> Material {
    Material {
        material_type: MaterialType::Obsidian,
        albedo: Vec3::new(0.02, 0.02, 0.03),
        reflectivity: 0.0,
        transparency: 0.0,
        refractive_index: 1.0,
        emissive: Vec3::zero(),
    }
}

pub fn get_pattern(position: &Vec3) -> Vec3 {
    let pixel_scale = 8.0;
    let pattern = pixelated_pattern(position, pixel_scale);
    let fine_pattern = pixelated_pattern(position, pixel_scale * 2.0);
    let coarse_pattern = pixelated_pattern(position, pixel_scale * 0.5);
    
    let base_dark = 0.02;
    let base_purple = 0.08;
    let base_black = 0.01;
    
    let variation = pattern * 0.1 + fine_pattern * 0.05 + coarse_pattern * 0.03;
    
    Vec3::new(
        base_black + variation * 0.5,
        base_black + variation * 0.3,
        base_dark + variation + base_purple
    )
}
