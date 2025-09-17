mod math;
mod materials;
mod camera;
mod scene;
mod app;
mod render;
mod input;

use app::App;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::time::Instant;
use rayon::ThreadPoolBuilder;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Error> {
    // Configure Rayon to use all available CPU cores
    let num_cpus = std::thread::available_parallelism().unwrap().get();
    ThreadPoolBuilder::new()
        .num_threads(num_cpus)
        .build_global()
        .expect("Failed to build thread pool");
    
    println!("🎮 Raytracer de Minecraft");
    println!("🖥️  Usando {} núcleos de CPU para renderizado paralelo", num_cpus);
    println!("Controles:");
    println!("  WASD - Mover cámara");
    println!("  Espacio/Shift - Subir/bajar");
    println!("  Arrastrar ratón - Mirar alrededor");
    println!("  ESC - Salir");
    println!();
    
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Raytracer de Minecraft")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(200, 150, surface_texture)?
    };

    let mut app = App::new();
    
    println!("Escena inicializada con {} cubos", app.scene.cubes.len());
    println!("Iniciando renderizado interactivo...");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Solicitud de cierre de ventana");
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event, .. } => {
                // Handle ESC key for exit
                if let WindowEvent::KeyboardInput { input: key_event, .. } = &event {
                    if let Some(VirtualKeyCode::Escape) = key_event.virtual_keycode {
                        if key_event.state == ElementState::Pressed {
                            println!("ESC presionado - saliendo");
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                    }
                }
                app.handle_input(&event);
            }
            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let delta_time = now.duration_since(app.last_frame_time).as_secs_f32();
                app.last_frame_time = now;
                
                app.update(delta_time);
                app.render(pixels.frame_mut());
                
                if pixels.render().is_err() {
                     *control_flow = ControlFlow::Exit;
                     return;
                 }
                
                // Actualizar título de ventana con FPS
                if app.frame_count % 30 == 0 {
                    window.set_title(&format!("Raytracer de Minecraft - {:.1} FPS", app.current_fps));
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
         }
     })
}
