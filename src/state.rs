use crate::renderer::{
    adapter_builder::AdapterBuilder, command_encoder_builder::CommandEncoderBuilder, device_builder::DeviceBuilder,
    instance_builder::InstanceBuilder, pipeline_builder::PiplineBuilder, surface_builder::SurfaceBuilder,
};
use wgpu::{util::DeviceExt, RenderPipeline};
use winit::{dpi::PhysicalSize, window::Window};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];

    pub fn description<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, 1.0], color: [0.0, 1.0] },
    Vertex { position: [1.0, 1.0], color: [1.0, 1.0] },
    Vertex { position: [1.0, -1.0], color: [1.0, 0.0] },
    Vertex { position: [-1.0, -1.0], color: [0.0, 0.0] },
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    2, 1, 0,
    3, 2, 0,
];

pub struct State<'a> {
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    index_buffer: wgpu::Buffer,
    queue: wgpu::Queue,
    size: PhysicalSize<u32>,
    render_pipeline: RenderPipeline,
    surface: wgpu::Surface<'a>,
    vertex_buffer: wgpu::Buffer,
    pub window: &'a Window,
}

impl<'a> State<'a> {
    pub async fn new(window: &'a Window) -> Self {
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

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let size = window.inner_size();

        let mut pipeline_builder = PiplineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vertex", "fragment");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build(&device);

        Self {
            config,
            device,
            index_buffer,
            queue,
            size,
            surface,
            render_pipeline,
            vertex_buffer,
            window,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_builder = CommandEncoderBuilder::new();
        let mut command_encoder = command_encoder_builder.build(&self.device);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        drop(render_pass);

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.size
    }
}
