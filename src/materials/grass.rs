use crate::math::Vec3;
use super::{Material, MaterialType, pixelated_pattern};

pub fn create() -> Material {
    Material {
        material_type: MaterialType::Grass,
        albedo: Vec3::new(0.25, 0.65, 0.25),
        reflectivity: 0.01,
        transparency: 0.0,
        refractive_index: 1.0,
        emissive: Vec3::zero(),
    }
}

pub fn get_pattern(position: &Vec3) -> Vec3 {
    let pixel_scale = 6.0;
    let pattern = pixelated_pattern(position, pixel_scale);
    let fine_pattern = pixelated_pattern(position, pixel_scale * 1.5);
    let coarse_pattern = pixelated_pattern(position, pixel_scale * 0.8);
    
    let base_green = 0.3;
    let bright_green = 0.7;
    let dark_green = 0.15;
    
    let variation = pattern * 0.4 + fine_pattern * 0.2 + coarse_pattern * 0.1;
    let darkness_factor = if pattern > 0.8 { 0.6 } else { 1.0 };
    let brightness_factor = if fine_pattern > 0.9 { 1.3 } else { 1.0 };
    
    Vec3::new(
        (dark_green + variation * 0.3) * darkness_factor * brightness_factor,
        (base_green + variation * bright_green) * darkness_factor * brightness_factor,
        (dark_green + variation * 0.2) * darkness_factor * brightness_factor
    )
}
