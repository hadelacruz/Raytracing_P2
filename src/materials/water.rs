use crate::math::Vec3;
use super::{Material, MaterialType};

pub fn create() -> Material {
    Material {
        material_type: MaterialType::Water,
        albedo: Vec3::new(0.2, 0.6, 1.0),
        reflectivity: 0.6,
        transparency: 0.8,
        refractive_index: 1.33,
        emissive: Vec3::zero(),
    }
}

pub fn get_pattern(position: &Vec3) -> Vec3 {
    let pixel_scale_x = 6.0;
    let pixel_scale_y = 12.0;
    let pixel_scale_z = 6.0;
    
    let x_pixel = (position.x * pixel_scale_x).floor();
    let y_pixel = (position.y * pixel_scale_y).floor();
    let z_pixel = (position.z * pixel_scale_z).floor();
    
    let hash = ((x_pixel * 73.0 + y_pixel * 37.0 + z_pixel * 17.0) % 100.0).abs() / 100.0;
    
    let base_cyan = 0.3;
    let bright_cyan = 0.9;
    let mid_cyan = 0.6;
    
    let (r, g, b) = if hash > 0.85 {
        (base_cyan * 0.6, bright_cyan, bright_cyan)
    } else if hash > 0.6 {
        (base_cyan * 0.7, mid_cyan, bright_cyan * 0.9)
    } else if hash > 0.3 {
        (base_cyan * 0.5, mid_cyan * 0.8, bright_cyan * 0.85)
    } else {
        (base_cyan * 0.4, base_cyan, mid_cyan)
    };
    
    Vec3::new(r, g, b)
}
