use crate::math::{Vec3, Ray};
use crate::materials::Material;

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub hit: bool,
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl HitInfo {
    pub fn miss() -> Self {
        HitInfo {
            hit: false,
            distance: f32::INFINITY,
            point: Vec3::zero(),
            normal: Vec3::zero(),
            material: Material::stone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cube {
    pub center: Vec3,
    pub size: Vec3,
    pub material: Material,
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere { center, radius, material }
    }

    pub fn intersect(&self, ray: &Ray, time: f32) -> HitInfo {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return HitInfo::miss();
        }

        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);

        let t = if t1 > 0.001 { t1 } else if t2 > 0.001 { t2 } else { return HitInfo::miss(); };

        let hit_point = ray.at(t);
        let normal = (hit_point - self.center).normalize();

        HitInfo {
            hit: true,
            distance: t,
            point: hit_point,
            normal,
            material: self.material.get_animated_properties(time, &hit_point),
        }
    }
}

impl Cube {
    pub fn new(center: Vec3, size: Vec3, material: Material) -> Self {
        Cube { center, size, material }
    }

    pub fn intersect(&self, ray: &Ray, time: f32) -> HitInfo {
        let min = self.center - self.size * 0.5;
        let max = self.center + self.size * 0.5;

        let inv_dir = Vec3::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);
        
        let t1 = (min.x - ray.origin.x) * inv_dir.x;
        let t2 = (max.x - ray.origin.x) * inv_dir.x;
        let t3 = (min.y - ray.origin.y) * inv_dir.y;
        let t4 = (max.y - ray.origin.y) * inv_dir.y;
        let t5 = (min.z - ray.origin.z) * inv_dir.z;
        let t6 = (max.z - ray.origin.z) * inv_dir.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 || tmin > tmax {
            return HitInfo::miss();
        }

        let t = if tmin > 0.001 { tmin } else { tmax };
        if t < 0.001 {
            return HitInfo::miss();
        }

        let hit_point = ray.at(t);
        let mut normal = Vec3::zero();

        // Determine which face was hit
        let eps = 0.001;
        if (hit_point.x - min.x).abs() < eps {
            normal = Vec3::new(-1.0, 0.0, 0.0);
        } else if (hit_point.x - max.x).abs() < eps {
            normal = Vec3::new(1.0, 0.0, 0.0);
        } else if (hit_point.y - min.y).abs() < eps {
            normal = Vec3::new(0.0, -1.0, 0.0);
        } else if (hit_point.y - max.y).abs() < eps {
            normal = Vec3::new(0.0, 1.0, 0.0);
        } else if (hit_point.z - min.z).abs() < eps {
            normal = Vec3::new(0.0, 0.0, -1.0);
        } else if (hit_point.z - max.z).abs() < eps {
            normal = Vec3::new(0.0, 0.0, 1.0);
        }

        HitInfo {
            hit: true,
            distance: t,
            point: hit_point,
            normal,
            material: self.material.get_animated_properties(time, &hit_point),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub cubes: Vec<Cube>,
    pub spheres: Vec<Sphere>,
    pub time: f32,
    pub sun_position: Vec3,
    pub sun_intensity: f32,
    pub ambient_light: Vec3,
    pub torch_positions: Vec<Vec3>,
    pub fire_particle_start_index: usize,
}

impl Scene {
    pub fn new() -> Self {
        let mut scene = Scene {
            cubes: Vec::new(),
            spheres: Vec::new(),
            time: 0.0,
            sun_position: Vec3::new(10.0, 10.0, 10.0),
            sun_intensity: 1.0,
            ambient_light: Vec3::new(0.3, 0.4, 0.6),
            torch_positions: Vec::new(),
            fire_particle_start_index: 0,
        };

        scene.create_minecraft_world();
        scene
    }

    // Define irregular water area with multiple corners and indentations
    fn is_water_position(&self, x: i32, z: i32) -> bool {
        // Create a more complex, irregular water shape
        // This creates a lake-like shape with multiple corners and curves
        
        // Main water body (larger than before)
        let main_area = (x >= 2 && x <= 5 && z >= 3 && z <= 5) ||
                        (x >= 3 && x <= 4 && z >= 2 && z <= 6) ||
                        (x >= 1 && x <= 5 && z >= 4 && z <= 4);
        
        // Add extensions and corners for irregular shape
        let extensions = 
            // Right extension
            (x == 6 && z >= 3 && z <= 4) ||
            // Top extension  
            (x >= 3 && x <= 4 && z == 1) ||
            // Bottom left corner
            (x == 1 && z == 5) ||
            // Bottom right bulge
            (x == 5 && z == 6) ||
            // Left side indentation fill
            (x == 2 && z == 6) ||
            // Top right corner
            (x == 5 && z == 2) ||
            // Additional irregular bits
            (x == 0 && z == 4) ||
            (x == 4 && z == 7 && z <= 5) || // This condition will be false, keeping for structure
            // More organic connections
            (x == 2 && z == 2) ||
            (x == 6 && z == 5);
            
        main_area || extensions
    }

    fn create_minecraft_world(&mut self) {
        // Ground plane with grass on top
        for x in -4..5 {
            for z in -4..5 {
                // Check if this position is part of the irregular water area
                let is_water_area = self.is_water_position(x, z);
                
                // Dirt/stone base
                self.cubes.push(Cube::new(
                    Vec3::new(x as f32 * 2.0, -1.0, z as f32 * 2.0),
                    Vec3::new(2.0, 0.4, 2.0),
                    Material::stone(),
                ));
                
                // Grass layer on top (skip in water area)
                if !is_water_area {
                    self.cubes.push(Cube::new(
                        Vec3::new(x as f32 * 2.0, -0.7, z as f32 * 2.0),
                        Vec3::new(2.0, 0.2, 2.0),
                        Material::grass(),
                    ));
                }
            }
        }

        // Create irregular water area
        for x in -4..5 {
            for z in -4..5 {
                if self.is_water_position(x, z) {
                    self.cubes.push(Cube::new(
                        Vec3::new(x as f32 * 2.0, -0.7, z as f32 * 2.0),
                        Vec3::new(2.0, 0.2, 2.0),
                        Material::water(),
                    ));
                }
            }
        }

        // Enhanced Nether Portal with obsidian frame - Moved to center
        // Portal frame (obsidian)
        for y in 0..5 {
            // Left pillar
            self.cubes.push(Cube::new(
                Vec3::new(-1.0, y as f32, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
                Material::obsidian(),
            ));
            // Right pillar
            self.cubes.push(Cube::new(
                Vec3::new(1.0, y as f32, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
                Material::obsidian(),
            ));
        }
        // Top and bottom frame
        for x in -1..2 {
            // Bottom
            self.cubes.push(Cube::new(
                Vec3::new(x as f32, -1.0, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
                Material::obsidian(),
            ));
            // Top
            self.cubes.push(Cube::new(
                Vec3::new(x as f32, 4.0, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
                Material::obsidian(),
            ));
        }
        // Portal interior (larger)
        for y in 0..4 {
            self.cubes.push(Cube::new(
                Vec3::new(0.0, y as f32, 0.0),
                Vec3::new(0.8, 1.0, 0.2),
                Material::portal(),
            ));
        }

        // Add a sun sphere in the sky
        self.spheres.push(Sphere::new(
            Vec3::new(15.0, 20.0, 10.0),
            2.0,
            Material::sun()
        ));

        // Campfire in the corner (wooden cross with fire particles)
        let campfire_x = -6.0; // Corner position
        let campfire_z = -6.0;
        
        // Wooden cross structure
        // Horizontal log
        for i in -1..2 {
            self.cubes.push(Cube::new(
                Vec3::new(campfire_x + i as f32, -0.3, campfire_z),
                Vec3::new(0.8, 0.3, 0.3),
                Material::wood(),
            ));
        }
        // Vertical log
        for i in -1..2 {
            self.cubes.push(Cube::new(
                Vec3::new(campfire_x, -0.3, campfire_z + i as f32),
                Vec3::new(0.3, 0.3, 0.8),
                Material::wood(),
            ));
        }
        
        // Mark where fire particles start in the spheres vector
        self.fire_particle_start_index = self.spheres.len();
        
        // Create dynamic fire particles as spheres (torch-like rays)
        let campfire_x = -6.0;
        let campfire_z = -6.0;
        
        // Create multiple fire particle "rays" with different trajectories
        for i in 0..8 {
            let angle = (i as f32) * 0.785; // ~45 degrees apart (2π/8)
            let distance = 0.3 + (i as f32 % 3.0) * 0.1; // Varying distances from center
            let height_offset = (i as f32 % 4.0) * 0.2; // Different starting heights
            
            // Create particles that will move in torch-like rays
            self.spheres.push(Sphere::new(
                Vec3::new(
                    campfire_x + angle.cos() * distance,
                    0.1 + height_offset,
                    campfire_z + angle.sin() * distance
                ),
                0.08, // Small radius for particle effect
                Material::fire_particle()
            ));
        }
        
        // Additional central fire particles for base flame
        for i in 0..4 {
            let height = i as f32 * 0.15;
            let wobble = (i as f32 * 0.5).sin() * 0.1;
            
            self.spheres.push(Sphere::new(
                Vec3::new(
                    campfire_x + wobble,
                    0.0 + height,
                    campfire_z + wobble * 0.5
                ),
                0.1 + (3 - i) as f32 * 0.02, // Larger at bottom, smaller at top
                Material::fire_particle()
            ));
        }

        let tree_x = -6.0;
        let tree_z = 6.0;   
        
        // Tree trunk (reduced height: 3 blocks instead of 4)
        for y in 0..3 {
            self.cubes.push(Cube::new(
                Vec3::new(tree_x, y as f32, tree_z),
                Vec3::new(0.8, 1.0, 0.8),
                Material::wood(),
            ));
        }
        
        // Irregular leaf canopy (like a small mountain of leaves) - restored to full size
        // Bottom layer of leaves (widest)
        for x_offset in -2i32..3i32 {
            for z_offset in -2i32..3i32 {
                let distance_from_center = (x_offset.abs() + z_offset.abs()) as f32;
                // Create irregular shape - not all positions have leaves
                if distance_from_center <= 3.0 && 
                   !(x_offset.abs() == 2 && z_offset.abs() == 2) { // Remove corners
                    self.cubes.push(Cube::new(
                        Vec3::new(tree_x + x_offset as f32, 2.5, tree_z + z_offset as f32),
                        Vec3::new(1.0, 0.8, 1.0),
                        Material::leaves(),
                    ));
                }
            }
        }
        
        // Middle layer of leaves (medium size)
        for x_offset in -1i32..2i32 {
            for z_offset in -1i32..2i32 {
                let distance_from_center = (x_offset.abs() + z_offset.abs()) as f32;
                if distance_from_center <= 2.0 {
                    self.cubes.push(Cube::new(
                        Vec3::new(tree_x + x_offset as f32, 3.5, tree_z + z_offset as f32),
                        Vec3::new(1.0, 0.8, 1.0),
                        Material::leaves(),
                    ));
                }
            }
        }
        
        // Top layer of leaves (smallest)
        for x_offset in -1i32..2i32 {
            for z_offset in -1i32..2i32 {
                if x_offset.abs() <= 1 && z_offset.abs() <= 1 && 
                   !(x_offset.abs() == 1 && z_offset.abs() == 1) { // Remove diagonal corners
                    self.cubes.push(Cube::new(
                        Vec3::new(tree_x + x_offset as f32, 4.5, tree_z + z_offset as f32),
                        Vec3::new(1.0, 0.8, 1.0),
                        Material::leaves(),
                    ));
                }
            }
        }
        
        // Peak of the tree (single leaf block)
        self.cubes.push(Cube::new(
            Vec3::new(tree_x, 5.5, tree_z),
            Vec3::new(1.0, 0.8, 1.0),
            Material::leaves(),
        ));
        
        // No torch positions - torches have been removed
        self.torch_positions = vec![];
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
        
        // Update sun position for day-night cycle
        let day_cycle = self.time * 0.2; // Slow day-night cycle
        self.sun_position = Vec3::new(
            day_cycle.cos() * 15.0,
            (day_cycle.sin() * 10.0).max(2.0),
            10.0,
        );
        
        // Update sun intensity
        self.sun_intensity = (day_cycle.sin().max(0.0) * 0.8 + 0.2).max(0.1);
        
        // Update ambient light based on time of day
        let night_factor = (1.0 - self.sun_intensity).max(0.0);
        self.ambient_light = Vec3::new(
            0.05 + self.sun_intensity * 0.15,
            0.05 + self.sun_intensity * 0.15,
            0.1 + self.sun_intensity * 0.1 + night_factor * 0.05,
        );
        
        // Animate fire particles like torch rays
        let campfire_x = -6.0;
        let campfire_z = -6.0;
        
        // Update fire particle positions to create dynamic ray effect
        for i in self.fire_particle_start_index..self.spheres.len() {
            let particle_index = i - self.fire_particle_start_index;
            
            if particle_index < 8 {
                // Outer fire rays - move them in arcs and upward
                let base_angle = (particle_index as f32) * 0.785; // Base angle
                let time_offset = particle_index as f32 * 0.3; // Different timing for each ray
                let wave_time = self.time * 3.0 + time_offset;
                
                // Create swaying motion like torch flames
                let sway_x = (wave_time * 1.2).sin() * 0.2;
                let sway_z = (wave_time * 0.8).cos() * 0.15;
                let rise = (wave_time * 2.0).sin().abs() * 0.5 + 0.3; // Particles rise and fall
                
                // Reset particle position if it gets too high (recycling effect)
                let height = if rise > 1.0 { 0.1 } else { rise };
                
                let distance = 0.2 + (wave_time * 0.5).sin().abs() * 0.3;
                
                self.spheres[i].center = Vec3::new(
                    campfire_x + (base_angle + sway_x).cos() * distance,
                    height,
                    campfire_z + (base_angle + sway_z).sin() * distance
                );
                
                // Vary radius for flickering effect
                self.spheres[i].radius = 0.06 + (wave_time * 4.0).sin().abs() * 0.04;
                
            } else {
                // Central flame particles - vertical motion with wobble
                let central_index = particle_index - 8;
                let wobble_time = self.time * 4.0 + central_index as f32 * 0.5;
                
                let wobble_x = (wobble_time * 1.5).sin() * 0.08;
                let wobble_z = (wobble_time * 1.8).cos() * 0.06;
                let flicker_height = (wobble_time * 2.5).sin().abs() * 0.2 + central_index as f32 * 0.12;
                
                // Reset if too high
                let height = if flicker_height > 0.8 { 0.0 } else { flicker_height };
                
                self.spheres[i].center = Vec3::new(
                    campfire_x + wobble_x,
                    height,
                    campfire_z + wobble_z
                );
                
                // Size varies with height (bigger at bottom)
                self.spheres[i].radius = 0.08 + (0.8 - height) * 0.05 + (wobble_time * 6.0).sin().abs() * 0.02;
            }
        }
    }

    pub fn intersect(&self, ray: &Ray) -> HitInfo {
        let mut closest_hit = HitInfo::miss();
        
        for cube in &self.cubes {
            let hit = cube.intersect(ray, self.time);
            if hit.hit && hit.distance < closest_hit.distance {
                closest_hit = hit;
            }
        }
        
        for sphere in &self.spheres {
            let hit = sphere.intersect(ray, self.time);
            if hit.hit && hit.distance < closest_hit.distance {
                closest_hit = hit;
            }
        }
        
        closest_hit
    }

    pub fn get_sky_color(&self, direction: &Vec3) -> Vec3 {
        let t = (direction.y + 1.0) * 0.5;
        let day_sky = Vec3::new(0.5, 0.7, 1.0);
        let night_sky = Vec3::new(0.05, 0.05, 0.2);
        let horizon = Vec3::new(1.0, 0.8, 0.6);
        
        let sky_color = day_sky.lerp(&night_sky, 1.0 - self.sun_intensity);
        let final_color = horizon.lerp(&sky_color, t);
        
        // Add stars at night
        if self.sun_intensity < 0.3 {
            let star_noise = ((direction.x * 100.0).sin() * (direction.z * 100.0).cos()).abs();
            if star_noise > 0.99 {
                return final_color + Vec3::new(0.8, 0.8, 1.0) * (1.0 - self.sun_intensity);
            }
        }
        
        final_color
    }
}