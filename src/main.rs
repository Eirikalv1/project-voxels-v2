mod renderer;
use renderer::{
    adapter_builder::AdapterBuilder, device_builder::DeviceBuilder, instance_builder::InstanceBuilder, pipeline_builder::PiplineBuilder,
    surface_builder::SurfaceBuilder,
};

use wgpu::{util::DeviceExt, RenderPipeline};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoopBuilder,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: &'a Window,
    render_pipeline: RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> Self {
        let instance_builder = InstanceBuilder::new();
        let instance = instance_builder.build();

        let surface_builder: SurfaceBuilder = SurfaceBuilder::new();
        let surface = surface_builder.build(&instance, window);

        let adapter_builder = AdapterBuilder::new();
        let adapter = adapter_builder.build(&instance, &surface).await;

        let device_builder = DeviceBuilder::new();
        let (device, queue) = device_builder.build(&adapter).await;

        let capabilities = surface.get_capabilities(&adapter);
        let format = surface_builder.create_initial_format(&capabilities);
        let config = surface_builder.create_initial_configuration(&capabilities, &format, window);
        surface.configure(&device, &config);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let size = window.inner_size();

        let mut pipeline_builder = PiplineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vertex", "fragment");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build(&device);

        Self {
            window,
            surface,
            device,
            vertex_buffer,
            queue,
            config,
            size,
            render_pipeline,
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") };
        let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            ..Default::default()
        };

        let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);
        drop(render_pass);

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }
}

async fn run() {
    let event_loop = EventLoopBuilder::new().build().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent { window_id, ref event } if window_id == state.window.id() => match event {
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
                WindowEvent::RedrawRequested => match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    Err(e) => log::error!("SurfaceError: {:?}", e),
                },
                WindowEvent::Resized(new_size) => {
                    state.resize(*new_size);
                    state.window.request_redraw();
                }
                _ => (),
            },

            _ => {}
        })
        .unwrap();
}

fn main() {
    std::env::set_var("RUST_LOG", "error");
    pretty_env_logger::init();

    pollster::block_on(run());
}
