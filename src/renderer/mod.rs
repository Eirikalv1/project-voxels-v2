use egui_wgpu::ScreenDescriptor;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::{
    gui::{gui, EguiRenderer},
    GpuContext,
};

use self::pipeline_builder::PiplineBuilder;

mod pipeline_builder;

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

pub struct Renderer {
    frame_data_buffer: wgpu::Buffer,
    frame_data_bind_group: wgpu::BindGroup,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

impl Renderer {
    pub fn new(context: &GpuContext) -> Self {
        let vertex_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let frame_data_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
            label: Some("Frame data buffer"),
        });
        let frame_data_group_layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Frame data group layout"),
        });
        let frame_data_bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &frame_data_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: frame_data_buffer.as_entire_binding(),
            }],
            label: Some("Frame data bind group"),
        });

        let mut pipeline_builder = PiplineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(context.surface_config.format);
        let render_pipeline = pipeline_builder.build(&context.device, &frame_data_group_layout);

        Self {
            frame_data_buffer,
            frame_data_bind_group,
            index_buffer,
            render_pipeline,
            vertex_buffer,
        }
    }
    pub fn render(
        &self,
        context: &GpuContext,
        egui: &mut EguiRenderer,
        window: &Window,
        frametime: u128,
    ) -> Result<(), wgpu::SurfaceError> {
        let drawable = context.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = &wgpu::CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        };
        let mut command_encoder = context.device.create_command_encoder(command_encoder_descriptor);

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

        let frame_data: [f32; 2] = [1920.0, 1080.0];
        context
            .queue
            .write_buffer(&self.frame_data_buffer, 0, bytemuck::cast_slice(&frame_data));

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.frame_data_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        drop(render_pass);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [context.surface_config.width, context.surface_config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        let view = drawable.texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: None,
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        egui.draw(
            &context.device,
            &mut command_encoder,
            &context.queue,
            |ui| gui(ui, frametime),
            screen_descriptor,
            &window,
            &view,
        );

        context.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }
}
