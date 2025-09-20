use crate::{
    math::Vec3,
    camera::Camera,
    scene::Scene,
    render::Renderer,
    input::InputHandler,
};
use std::time::Instant;
use winit::event::WindowEvent;

const RENDER_WIDTH: usize = 200;
const RENDER_HEIGHT: usize = 150;

pub struct App {
    pub scene: Scene,
    camera: Camera,
    pub last_frame_time: Instant,
    renderer: Renderer,
    input_handler: InputHandler,
    
    // Performance tracking
    pub frame_count: u32,
    fps_timer: Instant,
    pub current_fps: f32,
}

impl App {
    pub fn new() -> Self {
        let aspect_ratio = RENDER_WIDTH as f32 / RENDER_HEIGHT as f32;
        let mut camera = Camera::new(aspect_ratio);
        camera.set_target(Vec3::new(0.0, 0.0, 0.0));
        
        App {
            scene: Scene::new(),
            camera,
            last_frame_time: Instant::now(),
            renderer: Renderer::new(),
            input_handler: InputHandler::new(),
            frame_count: 0,
            fps_timer: Instant::now(),
            current_fps: 0.0,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Update scene
        self.scene.update(delta_time);
        
        // Handle camera movement based on input
        let speed = 5.0 * delta_time;
        self.input_handler.handle_camera_input(&mut self.camera, speed, delta_time);
        
        // Update FPS counter
        self.frame_count += 1;
        if self.fps_timer.elapsed().as_secs_f32() >= 1.0 {
            self.current_fps = self.frame_count as f32 / self.fps_timer.elapsed().as_secs_f32();
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }
    }
    
    pub fn render(&self, frame: &mut [u8]) {
        self.renderer.render(frame, &self.scene, &self.camera, self.current_fps);
    }
    
    pub fn handle_input(&mut self, event: &WindowEvent) {
        // Handle mouse movement with camera directly
        if let WindowEvent::CursorMoved { position, .. } = event {
            self.input_handler.handle_mouse_input(&mut self.camera, (position.x, position.y));
        }
        
        // Handle other input events
        self.input_handler.handle_event(event);
    }
}