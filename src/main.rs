mod math;
mod materials;
mod camera;
mod scene;
mod framebuffer;
mod raytracer;

use minifb::{Key, Window, WindowOptions};
use framebuffer::Framebuffer;
use raytracer::Raytracer;
use camera::Camera;
use scene::Scene;
use std::time::Instant;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const RENDER_WIDTH: usize = 200;
const RENDER_HEIGHT: usize = 150;

fn main() {
    let mut window = Window::new(
        "Minecraft Raytracer",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).expect("No se pudo crear la ventana");

    window.set_target_fps(60);

    println!("Controles: WASD + Flechas + Mouse + ESC");

    let mut framebuffer = Framebuffer::new(RENDER_WIDTH, RENDER_HEIGHT);
    let aspect_ratio = RENDER_WIDTH as f32 / RENDER_HEIGHT as f32;
    let mut camera = Camera::new(aspect_ratio);
    camera.set_target(math::Vec3::new(0.0, 0.0, 0.0));

    let mut scene = Scene::new();
    let raytracer = Raytracer::new();

    let mut last_time = Instant::now();
    let mut fps_count = 0;
    let mut fps_start = Instant::now();

    let mut last_mouse_pos: Option<(f32, f32)> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let delta = Instant::now().duration_since(last_time).as_secs_f32();
        last_time = Instant::now();

        fps_count += 1;
        if fps_start.elapsed().as_secs() >= 1 {
            window.set_title(&format!("Maincraft - {} FPS", fps_count));
            fps_count = 0;
            fps_start = Instant::now();
        }

        let speed = 5.0 * delta;

        let forward = window.is_key_down(Key::W);
        let backward = window.is_key_down(Key::S);
        let left = window.is_key_down(Key::A);
        let right = window.is_key_down(Key::D);
        let up = window.is_key_down(Key::Up);
        let down = window.is_key_down(Key::Down);

        camera.handle_keyboard_input(forward, backward, left, right, up, down, speed);

        if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            if window.get_mouse_down(minifb::MouseButton::Left) {
                if let Some((last_x, last_y)) = last_mouse_pos {
                    let delta_x = mx - last_x;
                    let delta_y = my - last_y;
                    camera.handle_mouse_input(delta_x * 0.005, -delta_y * 0.005, 1.0);
                }
                last_mouse_pos = Some((mx, my));
            } else {
                last_mouse_pos = None;
            }
        }

        scene.update(delta);
        raytracer.render(&mut framebuffer, &scene, &camera);

        let window_buffer = framebuffer.to_u32_buffer();
        window.update_with_buffer(&window_buffer, RENDER_WIDTH, RENDER_HEIGHT).unwrap();
    }
}
