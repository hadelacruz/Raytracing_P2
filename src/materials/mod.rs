use crate::math::Vec3;

pub mod stone;
pub mod water;
pub mod wood;
pub mod grass;
pub mod obsidian;
pub mod sun;
pub mod fire_particle;
pub mod leaves;

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Stone,
    Water,
    Wood,
    Grass,
    Obsidian,
    Sun,
    FireParticle,
    Leaves,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub material_type: MaterialType,
    pub albedo: Vec3,
    pub reflectivity: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub emissive: Vec3,
}

impl Material {
    pub fn get_animated_properties(&self, time: f32, position: &Vec3) -> Material {
        let mut material = *self;
        
        match self.material_type {
            MaterialType::Water => {
                material.albedo = water::get_pattern(position);
            },
            MaterialType::Grass => {
                material.albedo = grass::get_pattern(position);
            },
            MaterialType::Obsidian => {
                material.albedo = obsidian::get_pattern(position);
            },
            MaterialType::Wood => {
                material.albedo = wood::get_pattern(position);
            },
            MaterialType::Leaves => {
                material.albedo = leaves::get_pattern(position);
            },
            MaterialType::FireParticle => {
                material.emissive = fire_particle::get_animated_emissive(time, position);
            },
            _ => {}
        }
        
        material
    }

    // Constructor functions
    pub fn stone() -> Self {
        stone::create()
    }

    pub fn water() -> Self {
        water::create()
    }

    pub fn wood() -> Self {
        wood::create()
    }

    pub fn grass() -> Self {
        grass::create()
    }

    pub fn obsidian() -> Self {
        obsidian::create()
    }

    pub fn sun() -> Self {
        sun::create()
    }

    pub fn fire_particle() -> Self {
        fire_particle::create()
    }

    pub fn leaves() -> Self {
        leaves::create()
    }
}

// Shared utility functions
pub(crate) fn pixelated_pattern(position: &Vec3, scale: f32) -> f32 {
    let x = (position.x * scale).floor();
    let y = (position.y * scale).floor();
    let z = (position.z * scale).floor();
    
    let hash = ((x * 73.0 + y * 37.0 + z * 17.0) % 100.0).abs();
    hash / 100.0
}
