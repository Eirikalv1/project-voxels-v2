use crate::{renderer::Renderer, FrameTimer, GpuContext};

use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoopBuilder,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub async fn run() {
    let event_loop = EventLoopBuilder::new().build().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800, 800))
        .build(&event_loop)
        .unwrap();

    let mut context = GpuContext::new(&window).await;
    let renderer = Renderer::new(&context);

    let mut frame_timer = FrameTimer::new();
    frame_timer.start();

    let window = &window;
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent { window_id, ref event } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            repeat: false,
                            ..
                        },
                    ..
                } => {
                    elwt.exit();
                }
                WindowEvent::RedrawRequested => match renderer.render(&context) {
                    Ok(_) => {
                        frame_timer.end();
                        //dbg!(frame_timer.delta_time());
                        frame_timer.start();
                        window.request_redraw()
                    }
                    Err(wgpu::SurfaceError::Lost) => elwt.exit(),
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    Err(e) => log::error!("Surface error: {:?}", e),
                },
                WindowEvent::Resized(new_size) => {
                    window.request_redraw();
                    context.resize_surface_config(new_size);
                }
                _ => (),
            },

            _ => {}
        })
        .expect("Event loop failed.");
}
