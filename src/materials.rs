use crate::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Stone,
    Water,
    Wood,
    Portal,
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
    pub roughness: f32,
}

impl Material {
    // Helper function to create pixelated patterns like Minecraft textures
    fn pixelated_pattern(position: &Vec3, scale: f32) -> f32 {
        let x = (position.x * scale).floor();
        let y = (position.y * scale).floor();
        let z = (position.z * scale).floor();
        
        // Create a pseudo-random pattern based on position
        let hash = ((x * 73.0 + y * 37.0 + z * 17.0) % 100.0).abs();
        hash / 100.0
    }
    
    // Generate Minecraft-style obsidian pattern
    fn obsidian_pattern(position: &Vec3) -> Vec3 {
        // Create a 16x16 pixel pattern (like Minecraft textures)
        let pixel_scale = 8.0; // Higher value = smaller pixels
        let pattern = Self::pixelated_pattern(position, pixel_scale);
        
        // Create multiple layers for more detail
        let fine_pattern = Self::pixelated_pattern(position, pixel_scale * 2.0);
        let coarse_pattern = Self::pixelated_pattern(position, pixel_scale * 0.5);
        
        // Mix different shades of dark colors for obsidian
        let base_dark = 0.02;
        let base_purple = 0.08;
        let base_black = 0.01;
        
        // Create variation in the obsidian color
        let variation = pattern * 0.1 + fine_pattern * 0.05 + coarse_pattern * 0.03;
        
        Vec3::new(
            base_black + variation * 0.5,           // Very dark red
            base_black + variation * 0.3,           // Very dark green  
            base_dark + variation + base_purple     // Slightly more blue/purple
        )
    }
    
    // Generate Minecraft-style grass pattern
    fn grass_pattern(position: &Vec3) -> Vec3 {
        // Create a 16x16 pixel pattern similar to Minecraft grass texture
        let pixel_scale = 6.0; // Slightly larger pixels than obsidian for grass texture
        let pattern = Self::pixelated_pattern(position, pixel_scale);
        
        // Create multiple layers for grass detail
        let fine_pattern = Self::pixelated_pattern(position, pixel_scale * 1.5);
        let coarse_pattern = Self::pixelated_pattern(position, pixel_scale * 0.8);
        
        // Minecraft grass colors - mix of different green shades
        let base_green = 0.3;
        let bright_green = 0.7;
        let dark_green = 0.15;
        
        // Create variation in grass color with different green tones
        let variation = pattern * 0.4 + fine_pattern * 0.2 + coarse_pattern * 0.1;
        
        // Some pixels should be darker (dirt showing through) or brighter (healthy grass)
        let darkness_factor = if pattern > 0.8 { 0.6 } else { 1.0 }; // Some dark patches
        let brightness_factor = if fine_pattern > 0.9 { 1.3 } else { 1.0 }; // Some bright spots
        
        Vec3::new(
            (dark_green + variation * 0.3) * darkness_factor * brightness_factor,     // Red component (minimal)
            (base_green + variation * bright_green) * darkness_factor * brightness_factor, // Green component (dominant)
            (dark_green + variation * 0.2) * darkness_factor * brightness_factor      // Blue component (minimal)
        )
    }

    // Generate Minecraft-style water pattern with long cyan pixels
    fn water_pattern(position: &Vec3) -> Vec3 {
        // Create thinner, more vertical pixels for water - more delicate look
        let pixel_scale_x = 6.0; // Thinner pixels (increased from 3.0)
        let pixel_scale_y = 12.0; // Even taller pixels for more vertical effect
        let pixel_scale_z = 6.0; // Thinner in Z too
        
        // Create pixelated pattern with different scales for x, y, z
        let x_pixel = (position.x * pixel_scale_x).floor();
        let y_pixel = (position.y * pixel_scale_y).floor();
        let z_pixel = (position.z * pixel_scale_z).floor();
        
        // Create hash for this pixel position
        let hash = ((x_pixel * 73.0 + y_pixel * 37.0 + z_pixel * 17.0) % 100.0).abs() / 100.0;
        
        // Create a secondary pattern for variation
        let fine_hash = ((x_pixel * 23.0 + y_pixel * 41.0 + z_pixel * 19.0) % 50.0).abs() / 50.0;
        
        // Special check for pure white pixels - make them rare but visible
        if hash > 0.95 {
            // Pure white pixels (5% chance)
            return Vec3::new(1.0, 1.0, 1.0);
        }
        
        // More vivid Minecraft water colors - brighter cyan/white vertical pixels
        let dark_cyan = 0.25;
        let medium_cyan = 0.45;
        let bright_cyan = 0.75;
        let almost_white = 0.9;
        
        // Create distinct pixel regions with more vivid colors
        let cyan_variation = if hash > 0.85 {
            almost_white // Very bright almost-white cyan pixels
        } else if hash > 0.65 {
            bright_cyan // Bright cyan pixels
        } else if hash > 0.35 {
            medium_cyan // Medium cyan pixels
        } else {
            dark_cyan // Darker cyan base
        };
        
        // Add some subtle variation within each pixel type
        let micro_variation = fine_hash * 0.06;
        
        Vec3::new(
            (dark_cyan * 0.2 + micro_variation * 0.15).clamp(0.0, 0.35),                    // Minimal red for cyan
            (dark_cyan + cyan_variation * 0.65 + micro_variation * 0.2).clamp(0.35, 0.95), // Strong cyan component
            (medium_cyan + cyan_variation + micro_variation * 0.25).clamp(0.65, 1.0)       // Very strong blue component
        )
    }

    // Generate Minecraft-style leaves pattern with dark and light green pixels
    fn leaves_pattern(position: &Vec3) -> Vec3 {
        // Create pixelated pattern similar to Minecraft leaves texture
        let pixel_scale = 7.0; // Good balance for leaf detail
        let pattern = Self::pixelated_pattern(position, pixel_scale);
        
        // Create multiple layers for more organic leaf variation
        let fine_pattern = Self::pixelated_pattern(position, pixel_scale * 1.8);
        let coarse_pattern = Self::pixelated_pattern(position, pixel_scale * 0.6);
        
        // Minecraft leaves colors - mix of dark and light greens
        let very_dark_green = 0.08;
        let dark_green = 0.15;
        let medium_green = 0.35;
        let bright_green = 0.65;
        let very_bright_green = 0.85;
        
        // Create distinct green variations like Minecraft leaves
        let green_type = pattern + fine_pattern * 0.5 + coarse_pattern * 0.3;
        
        let (r, g, b): (f32, f32, f32) = if green_type > 0.9 {
            // Very bright green pixels (rare)
            (very_dark_green * 0.3, very_bright_green, very_dark_green * 0.4)
        } else if green_type > 0.75 {
            // Bright green pixels
            (very_dark_green * 0.4, bright_green, very_dark_green * 0.5)
        } else if green_type > 0.45 {
            // Medium green pixels
            (very_dark_green * 0.5, medium_green, very_dark_green * 0.6)
        } else if green_type > 0.25 {
            // Dark green pixels
            (very_dark_green * 0.6, dark_green, very_dark_green * 0.7)
        } else {
            // Very dark green pixels (shadows)
            (very_dark_green * 0.4, very_dark_green, very_dark_green * 0.5)
        };
        
        // Add some brown hints for more realistic leaves (some dying leaves)
        let brown_hint: f32 = if fine_pattern > 0.92 { 0.1 } else { 0.0 };
        
        Vec3::new(
            (r + brown_hint * 0.3).clamp(0.0, 0.4),
            g.clamp(0.05, 1.0),
            (b + brown_hint * 0.1).clamp(0.0, 0.4)
        )
    }

    pub fn stone() -> Self {
        Material {
            material_type: MaterialType::Stone,
            albedo: Vec3::new(0.6, 0.6, 0.6),
            reflectivity: 0.1,
            transparency: 0.0,
            refractive_index: 1.0,
            emissive: Vec3::zero(),
            roughness: 0.8,
        }
    }

    pub fn water() -> Self {
        Material {
            material_type: MaterialType::Water,
            albedo: Vec3::new(0.2, 0.6, 1.0), // Color más vivo: menos rojo, más verde y azul intenso
            reflectivity: 0.6,
            transparency: 0.8,
            refractive_index: 1.33,
            emissive: Vec3::zero(),
            roughness: 0.1,
        }
    }

    pub fn wood() -> Self {
        Material {
            material_type: MaterialType::Wood,
            albedo: Vec3::new(0.6, 0.4, 0.2),
            reflectivity: 0.05,
            transparency: 0.0,
            refractive_index: 1.0,
            emissive: Vec3::zero(),
            roughness: 0.9,
        }
    }

    pub fn portal() -> Self {
        Material {
            material_type: MaterialType::Portal,
            albedo: Vec3::new(0.5, 0.2, 0.8),
            reflectivity: 0.3,
            transparency: 0.2,
            refractive_index: 1.2,
            emissive: Vec3::new(1.0, 0.3, 2.0),
            roughness: 0.2,
        }
    }

    pub fn grass() -> Self {
        Material {
            material_type: MaterialType::Grass,
            albedo: Vec3::new(0.25, 0.65, 0.25), // More realistic grass green
            reflectivity: 0.01, // Mucho menos reflectivo - hierba más mate
            transparency: 0.0,
            refractive_index: 1.0,
            emissive: Vec3::zero(),
            roughness: 0.85, // Slightly less rough for better light interaction
        }
    }

    pub fn obsidian() -> Self {
        Material {
            material_type: MaterialType::Obsidian,
            albedo: Vec3::new(0.08, 0.05, 0.12), // Slightly purple-tinted dark base
            reflectivity: 0.0, // More reflective like real obsidian
            transparency: 0.0,
            refractive_index: 1.0,
            emissive: Vec3::zero(),
            roughness: 0.3, // Smoother surface like polished obsidian
        }
    }

    pub fn sun() -> Self {
        Material {
            material_type: MaterialType::Sun,
            albedo: Vec3::new(1.0, 0.9, 0.7),
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            emissive: Vec3::new(5.0, 4.0, 2.0),
            roughness: 1.0,
        }
    }

    pub fn fire_particle() -> Self {
        Material {
            material_type: MaterialType::FireParticle,
            albedo: Vec3::new(1.0, 0.6, 0.1),
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            emissive: Vec3::new(3.0, 1.5, 0.3),
            roughness: 1.0,
        }
    }

    pub fn leaves() -> Self {
        Material {
            material_type: MaterialType::Leaves,
            albedo: Vec3::new(0.2, 0.5, 0.2), // Base green color for leaves
            reflectivity: 0.02, // Very low reflectivity like real leaves
            transparency: 0.0,
            refractive_index: 1.0,
            emissive: Vec3::zero(),
            roughness: 0.9, // Rough surface like real leaves
        }
    }

    pub fn get_animated_properties(&self, time: f32, position: &Vec3) -> Material {
        let mut material = *self;
        
        match self.material_type {
            MaterialType::Water => {
                // Apply pixelated Minecraft-style water pattern
                let base_water_color = Self::water_pattern(position);
                
                // If it's a white pixel, keep it mostly white with minimal animation
                if base_water_color.x > 0.9 && base_water_color.y > 0.9 && base_water_color.z > 0.9 {
                    // Pure white pixels - keep them white with minimal variation
                    let tiny_wave = (time * 1.0 + position.x * 0.1 + position.z * 0.1).sin() * 0.02;
                    material.albedo = Vec3::new(
                        (1.0 + tiny_wave * 0.01).clamp(0.95, 1.0),
                        (1.0 + tiny_wave * 0.01).clamp(0.95, 1.0),
                        (1.0 + tiny_wave * 0.01).clamp(0.95, 1.0),
                    );
                } else {
                    // Regular water pixels with normal animation
                    let wave = (time * 1.5 + position.x * 0.3 + position.z * 0.2).sin() * 0.04;
                    material.albedo = Vec3::new(
                        (base_water_color.x + wave * 0.03).clamp(0.0, 0.4),
                        (base_water_color.y + wave * 0.05).clamp(0.3, 0.95),
                        (base_water_color.z + wave * 0.04).clamp(0.6, 1.0),
                    );
                }
                
                // Slight reflectivity variation based on water pattern
                let pattern_variation = Self::pixelated_pattern(position, 4.0);
                material.reflectivity = 0.5 + pattern_variation * 0.25;
            },
            MaterialType::Portal => {
                // Pulsing portal effect
                let pulse = (time * 3.0).sin() * 0.3 + 0.7;
                material.emissive = Vec3::new(
                    1.0 * pulse,
                    0.3 * pulse,
                    2.0 * pulse,
                );
                material.transparency = 0.2 + pulse * 0.3;
            },
            MaterialType::Obsidian => {
                // Apply pixelated Minecraft-style pattern to obsidian
                material.albedo = Self::obsidian_pattern(position);
                
                // Add subtle reflection variations based on the pattern
                let pattern_variation = Self::pixelated_pattern(position, 4.0);
                material.reflectivity = 0.15 + pattern_variation * 0.15;
            },
            MaterialType::Grass => {
                // Apply pixelated Minecraft-style grass pattern
                let base_grass_color = Self::grass_pattern(position);
                
                // Add subtle wind animation to make the grass feel alive
                let wind = (time * 1.5 + position.x * 0.3 + position.z * 0.2).sin() * 0.03;
                material.albedo = Vec3::new(
                    (base_grass_color.x + wind * 0.1).clamp(0.0, 1.0),
                    (base_grass_color.y + wind * 0.08).clamp(0.0, 1.0),
                    (base_grass_color.z + wind * 0.05).clamp(0.0, 1.0),
                );
                
                // Slight reflectivity variation based on grass pattern
                let pattern_variation = Self::pixelated_pattern(position, 5.0);
                material.reflectivity = 0.01 + pattern_variation * 0.02; // Muy baja reflectividad
            },
            MaterialType::FireParticle => {
                // Fire particle flickering animation
                let flicker_speed = 8.0;
                let flicker_intensity = 0.5;
                let flicker = (time * flicker_speed + position.x * 2.0 + position.y * 3.0).sin() * flicker_intensity + 1.0;
                
                material.emissive = Vec3::new(
                    3.0 * flicker,
                    1.5 * flicker * 0.8,
                    0.3 * flicker * 0.6,
                );
            },
            MaterialType::Leaves => {
                // Apply pixelated Minecraft-style leaves pattern
                let base_leaves_color = Self::leaves_pattern(position);
                
                // Add subtle wind animation to make the leaves feel alive
                let wind = (time * 1.2 + position.x * 0.4 + position.z * 0.3).sin() * 0.02;
                let rustle = (time * 2.5 + position.y * 0.5).cos() * 0.015;
                
                material.albedo = Vec3::new(
                    (base_leaves_color.x + wind * 0.05 + rustle * 0.03).clamp(0.0, 0.6),
                    (base_leaves_color.y + wind * 0.04 + rustle * 0.02).clamp(0.05, 1.0),
                    (base_leaves_color.z + wind * 0.06 + rustle * 0.04).clamp(0.0, 0.5),
                );
                
                // Slight reflectivity variation based on leaves pattern
                let pattern_variation = Self::pixelated_pattern(position, 6.0);
                material.reflectivity = 0.01 + pattern_variation * 0.03;
            },
            MaterialType::Sun => {
                // Pulsing sun intensity
                let pulse = (time * 2.0).sin() * 0.2 + 1.0;
                material.emissive = Vec3::new(
                    8.0 * pulse,
                    6.0 * pulse,
                    4.0 * pulse,
                );
            },
            _ => {}
        }
        
        material
    }
}