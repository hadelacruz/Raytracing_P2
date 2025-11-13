use crate::math::{Vec3, Ray};
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    
    // Camera controls
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Camera {
            position: Vec3::new(0.0, 5.0, 10.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 45.0,
            aspect_ratio,
            near: 0.1,
            far: 100.0,
            yaw: 0.0,
            pitch: 0.3,
            distance: 15.0,
            zoom: 1.0,
        }
    }

    pub fn update_position(&mut self) {
        let x = self.distance * self.pitch.cos() * self.yaw.sin();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.cos();
        
        self.position = self.target + Vec3::new(x, y, z);
    }

    pub fn get_ray(&self, x: f32, y: f32, width: f32, height: f32) -> Ray {
        let aspect = width / height;
        let fov_rad = (self.fov / self.zoom).to_radians();
        let half_height = (fov_rad / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (self.position - self.target).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);

        let lower_left_corner = self.position - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        let s = x / width;
        let t = y / height;

        let direction = lower_left_corner + s * horizontal + t * vertical - self.position;
        Ray::new(self.position, direction)
    }

    pub fn handle_mouse_input(&mut self, delta_x: f32, delta_y: f32, sensitivity: f32) {
        self.yaw += delta_x * sensitivity;
        self.pitch += delta_y * sensitivity;
        self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        self.update_position();
    }

    pub fn handle_keyboard_input(&mut self, forward: bool, backward: bool, left: bool, right: bool, up: bool, down: bool, speed: f32) {
        let w = (self.position - self.target).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);

        let mut movement = Vec3::zero();
        
        if forward {
            movement = movement - w * speed;
        }
        if backward {
            movement = movement + w * speed;
        }
        if left {
            movement = movement - u * speed;
        }
        if right {
            movement = movement + u * speed;
        }
        if up {
            movement = movement + v * speed;
        }
        if down {
            movement = movement - v * speed;
        }

        self.position = self.position + movement;
        self.target = self.target + movement;
    }

    pub fn set_target(&mut self, target: Vec3) {
        self.target = target;
        self.update_position();
    }

    pub fn orbit_around_target(&mut self, delta_time: f32, speed: f32) {
        self.yaw += delta_time * speed;
        self.update_position();
    }
}