use crate::{
    math::{Vec3, Ray},
    camera::Camera,
    scene::Scene,
};
use rayon::prelude::*;

const RENDER_WIDTH: usize = 200;
const RENDER_HEIGHT: usize = 150;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer
    }
    
    pub fn render(&self, frame: &mut [u8], scene: &Scene, camera: &Camera) {
        // Use parallel processing to render pixels across all CPU cores with optimal chunk size
        let chunk_size = (RENDER_WIDTH * RENDER_HEIGHT) / (std::thread::available_parallelism().unwrap().get() * 4);
        let chunk_size = chunk_size.max(1).min(1024); // Ensure reasonable chunk size
        
        frame.par_chunks_exact_mut(4)
            .with_min_len(chunk_size)
            .enumerate()
            .for_each(|(i, pixel)| {
                let x = i % RENDER_WIDTH;
                let y = i / RENDER_WIDTH;
                
                let color = self.render_pixel(x, y, scene, camera);
                
                // Convert to RGBA with gamma correction for better visual quality
                let r = (color.x.clamp(0.0, 1.0).sqrt() * 255.0) as u8;
                let g = (color.y.clamp(0.0, 1.0).sqrt() * 255.0) as u8;
                let b = (color.z.clamp(0.0, 1.0).sqrt() * 255.0) as u8;
                
                pixel[0] = r; // Red
                pixel[1] = g; // Green
                pixel[2] = b; // Blue
                pixel[3] = 255; // Alpha
            });
    }
    
    fn render_pixel(&self, x: usize, y: usize, scene: &Scene, camera: &Camera) -> Vec3 {
        let ray = camera.get_ray(
            x as f32 + 0.5,
            (RENDER_HEIGHT - 1 - y) as f32 + 0.5,
            RENDER_WIDTH as f32,
            RENDER_HEIGHT as f32,
        );
        
        self.trace_ray(&ray, 6, scene)
    }
    
    fn trace_ray(&self, ray: &Ray, depth: i32, scene: &Scene) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }

        let hit = scene.intersect(ray);
        if !hit.hit {
            return scene.get_sky_color(&ray.direction);
        }

        let mut color = scene.ambient_light * hit.material.albedo;

        // Direct lighting from sun
        let light_dir = (scene.sun_position - hit.point).normalize();
        let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, light_dir);
        let shadow_hit = scene.intersect(&shadow_ray);
        
        if !shadow_hit.hit || shadow_hit.distance > (scene.sun_position - hit.point).length() {
            let light_intensity = light_dir.dot(&hit.normal).max(0.0) * scene.sun_intensity;
            color = color + hit.material.albedo * light_intensity * Vec3::new(1.0, 0.9, 0.7);
        }

        // Emissive lighting
        for cube in &scene.cubes {
            if cube.material.emissive.length() > 0.0 {
                let light_pos = cube.center;
                let light_dir = (light_pos - hit.point).normalize();
                let distance = (light_pos - hit.point).length();
                let attenuation = 1.0 / (1.0 + 0.1 * distance + 0.01 * distance * distance);
                
                let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, light_dir);
                let shadow_hit = scene.intersect(&shadow_ray);
                
                if !shadow_hit.hit || shadow_hit.distance > distance {
                    let light_intensity = light_dir.dot(&hit.normal).max(0.0) * attenuation;
                    let animated_material = cube.material.get_animated_properties(scene.time, &cube.center);
                    color = color + hit.material.albedo * animated_material.emissive * light_intensity;
                }
            }
        }

        // Add emissive contribution
        color = color + hit.material.emissive;

        // Reflection
        if hit.material.reflectivity > 0.0 {
            let reflected_dir = ray.direction.reflect(&hit.normal);
            let reflected_ray = Ray::new(hit.point + hit.normal * 0.001, reflected_dir);
            let reflected_color = self.trace_ray(&reflected_ray, depth - 1, scene);
            color = color.lerp(&reflected_color, hit.material.reflectivity);
        }

        // Refraction
        if hit.material.transparency > 0.0 {
            let eta = if ray.direction.dot(&hit.normal) < 0.0 {
                1.0 / hit.material.refractive_index
            } else {
                hit.material.refractive_index
            };
            
            let normal = if ray.direction.dot(&hit.normal) < 0.0 {
                hit.normal
            } else {
                hit.normal * -1.0
            };
            
            if let Some(refracted_dir) = ray.direction.refract(&normal, eta) {
                let refracted_ray = Ray::new(hit.point - normal * 0.001, refracted_dir);
                let refracted_color = self.trace_ray(&refracted_ray, depth - 1, scene);
                color = color.lerp(&refracted_color, hit.material.transparency);
            }
        }

        color
    }
}