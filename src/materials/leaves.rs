use crate::math::Vec3;
use super::{Material, MaterialType, pixelated_pattern};

pub fn create() -> Material {
    Material {
        material_type: MaterialType::Leaves,
        albedo: Vec3::new(0.2, 0.5, 0.2),
        reflectivity: 0.02,
        transparency: 0.0,
        refractive_index: 1.0,
        emissive: Vec3::zero(),
    }
}

pub fn get_pattern(position: &Vec3) -> Vec3 {
    let pixel_scale = 5.0;
    let pattern = pixelated_pattern(position, pixel_scale);
    let fine_pattern = pixelated_pattern(position, pixel_scale * 2.0);
    let coarse_pattern = pixelated_pattern(position, pixel_scale * 0.7);
    
    let base_green = 0.15;
    let mid_green = 0.35;
    let bright_green = 0.55;
    let dark_green = 0.08;
    
    let variation = pattern * 0.5 + fine_pattern * 0.3 + coarse_pattern * 0.2;
    
    let (r, g, b) = if pattern > 0.88 {
        (bright_green * 0.4, bright_green, bright_green * 0.3)
    } else if pattern > 0.65 {
        (mid_green * 0.35, mid_green, mid_green * 0.25)
    } else if pattern > 0.35 {
        (base_green * 0.3, base_green + variation * 0.3, base_green * 0.2)
    } else {
        (dark_green * 0.25, dark_green, dark_green * 0.15)
    };
    
    let brown_hint: f32 = if fine_pattern > 0.92 { 0.1 } else { 0.0 };
    
    Vec3::new(
        (r + brown_hint * 0.3).clamp(0.0, 0.4),
        g.clamp(0.05, 1.0),
        (b + brown_hint * 0.1).clamp(0.0, 0.4)
    )
}
