use crate::camera::Camera;
use winit::event::{WindowEvent, ElementState, VirtualKeyCode, MouseButton};
use std::collections::HashSet;

pub struct InputHandler {
    keys_pressed: HashSet<VirtualKeyCode>,
    mouse_pressed: bool,
    last_mouse_pos: (f64, f64),
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            keys_pressed: HashSet::new(),
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
        }
    }
    
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input: key_event,
                ..
            } => {
                if let Some(keycode) = key_event.virtual_keycode {
                    match key_event.state {
                        ElementState::Pressed => {
                            self.keys_pressed.insert(keycode);
                        }
                        ElementState::Released => {
                            self.keys_pressed.remove(&keycode);
                        }
                    }
                }
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.mouse_pressed {
                    let _delta_x = position.x - self.last_mouse_pos.0;
                    let _delta_y = position.y - self.last_mouse_pos.1;
                    
                    // Handle mouse movement immediately here since we have the position
                    // This will be called from App with camera reference
                }
                self.last_mouse_pos = (position.x, position.y);
            }
            _ => {}
        }
    }
    
    pub fn handle_camera_input(&mut self, camera: &mut Camera, speed: f32, _delta_time: f32) {
        // Handle movement keys
        let forward = self.keys_pressed.contains(&VirtualKeyCode::W);
        let backward = self.keys_pressed.contains(&VirtualKeyCode::S);
        let left = self.keys_pressed.contains(&VirtualKeyCode::A);
        let right = self.keys_pressed.contains(&VirtualKeyCode::D);
        let up = self.keys_pressed.contains(&VirtualKeyCode::Up);      // Changed from Space to Up arrow
        let down = self.keys_pressed.contains(&VirtualKeyCode::Down);  // Changed from LShift to Down arrow
        
        camera.handle_keyboard_input(forward, backward, left, right, up, down, speed);
    }
    
    pub fn handle_mouse_input(&mut self, camera: &mut Camera, current_pos: (f64, f64)) {
        if self.mouse_pressed {
            let delta_x = current_pos.0 - self.last_mouse_pos.0;
            let delta_y = current_pos.1 - self.last_mouse_pos.1;
            
            camera.handle_mouse_input(
                delta_x as f32 * 0.005,
                -delta_y as f32 * 0.005,
                1.0,
            );
        }
        self.last_mouse_pos = current_pos;
    }
}