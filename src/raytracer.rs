use crate::{
    math::{Vec3, Ray},
    camera::Camera,
    scene::Scene,
    framebuffer::Framebuffer,
};
use rayon::prelude::*;

pub struct Raytracer {
    max_depth: i32,
}

impl Raytracer {
    pub fn new() -> Self {
        Raytracer { max_depth: 3 }
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, scene: &Scene, camera: &Camera) {
        framebuffer.clear();

        let pixels: Vec<(usize, usize, Vec3)> = (0..framebuffer.height)
            .into_par_iter()
            .flat_map(|y| {
                (0..framebuffer.width)
                    .map(|x| {
                        let color = self.render_pixel(x, y, framebuffer.width, framebuffer.height, scene, camera);
                        (x, y, color)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for (x, y, color) in pixels {
            framebuffer.set_pixel(x, y, color);
        }
    }

    fn render_pixel(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        scene: &Scene,
        camera: &Camera,
    ) -> Vec3 {
        let ray = camera.get_ray(
            x as f32 + 0.5,
            (height - 1 - y) as f32 + 0.5,
            width as f32,
            height as f32,
        );

        self.trace_ray(&ray, self.max_depth, scene)
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

        for cube in &scene.cubes {
            if cube.material.emissive.length() > 0.0 {
                let light_pos = cube.center;
                let distance = (light_pos - hit.point).length();
                
                if distance > 15.0 {
                    continue;
                }
                
                let light_dir = (light_pos - hit.point).normalize();
                let base_att = 1.0 / (1.0 + 0.05 * distance + 0.005 * distance * distance);
                let night_boost = 1.0 + (1.0 - scene.sun_intensity) * 0.8;
                let attenuation = base_att * night_boost;

                let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, light_dir);
                let shadow_hit = scene.intersect(&shadow_ray);

                if !shadow_hit.hit || shadow_hit.distance > distance {
                    let light_intensity = light_dir.dot(&hit.normal).max(0.0) * attenuation;
                    let animated_material = cube.material.get_animated_properties(scene.time, &cube.center);
                    color = color + hit.material.albedo * animated_material.emissive * light_intensity;
                }
            }
        }

        // Emissive lighting from spheres (fire particles)
        for sphere in &scene.spheres {
            if sphere.material.emissive.length() > 0.0 {
                let light_pos = sphere.center;
                let distance = (light_pos - hit.point).length();
                
                // Skip if light is too far
                if distance > 12.0 {
                    continue;
                }
                
                let light_dir = (light_pos - hit.point).normalize();

                // Enhanced attenuation for sphere lights with night boost
                let base_attenuation = 1.0 / (1.0 + 0.08 * distance + 0.008 * distance * distance);
                let night_boost = 1.0 + (1.0 - scene.sun_intensity) * 1.2;
                let attenuation = base_attenuation * night_boost;

                let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, light_dir);
                let shadow_hit = scene.intersect(&shadow_ray);

                if !shadow_hit.hit || shadow_hit.distance > distance {
                    let light_intensity = light_dir.dot(&hit.normal).max(0.0) * attenuation;
                    let animated_material = sphere.material.get_animated_properties(scene.time, &sphere.center);
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
