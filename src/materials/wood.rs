use crate::math::Vec3;
use super::{Material, MaterialType};

pub fn create() -> Material {
    Material {
        material_type: MaterialType::Wood,
        albedo: Vec3::new(0.35, 0.25, 0.15),
        reflectivity: 0.02,
        transparency: 0.0,
        refractive_index: 1.0,
        emissive: Vec3::zero(),
    }
}

pub fn get_pattern(position: &Vec3) -> Vec3 {
    let pixel_scale_x = 8.0;
    let pixel_scale_y = 16.0;
    let pixel_scale_z = 8.0;
    
    let x_pixel = (position.x * pixel_scale_x).floor();
    let y_pixel = (position.y * pixel_scale_y).floor();
    let z_pixel = (position.z * pixel_scale_z).floor();
    
    let hash = ((x_pixel * 73.0 + y_pixel * 37.0 + z_pixel * 17.0) % 100.0).abs() / 100.0;
    let grain_hash = ((x_pixel * 23.0 + z_pixel * 19.0) % 50.0).abs() / 50.0;
    
    let very_dark_brown = 0.15;
    let dark_brown = 0.25;
    let medium_brown = 0.35;
    let light_brown = 0.55;
    let beige = 0.75;
    let light_beige = 0.9;
    
    let (r, g, b): (f32, f32, f32) = if hash > 0.92 && grain_hash > 0.7 {
        (light_beige, light_beige * 0.9, light_beige * 0.7)
    } else if hash > 0.85 && grain_hash > 0.5 {
        (beige, beige * 0.85, beige * 0.65)
    } else if hash > 0.75 {
        (light_brown, light_brown * 0.75, light_brown * 0.5)
    } else if hash > 0.45 {
        (medium_brown, medium_brown * 0.7, medium_brown * 0.45)
    } else if hash > 0.25 {
        (dark_brown, dark_brown * 0.65, dark_brown * 0.4)
    } else {
        (very_dark_brown, very_dark_brown * 0.6, very_dark_brown * 0.35)
    };
    
    let micro_variation = (grain_hash * 0.1) - 0.05;
    
    Vec3::new(
        (r + micro_variation * 0.15).clamp(0.1, 1.0),
        (g + micro_variation * 0.12).clamp(0.05, 0.95),
        (b + micro_variation * 0.1).clamp(0.03, 0.8)
    )
}
