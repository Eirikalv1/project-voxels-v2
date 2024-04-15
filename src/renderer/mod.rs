use winit::window::Window;

use crate::{
    camera::{Camera, CameraUniform},
    gui::{gui, EguiRenderer},
    GpuContext,
};

use self::{
    containers::{BindGroupContainer, BufferContainer},
    pipeline_builder::PiplineBuilder,
};

pub mod containers;
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
    bind_groups: BindGroupContainer,
    buffers: BufferContainer,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(context: &GpuContext) -> Self {
        let bind_group_layouts: [&wgpu::BindGroupLayout; 2];

        let mut buffers = BufferContainer::new();
        let mut bind_groups = BindGroupContainer::new();

        buffers.create_vertex_buffer_init(bytemuck::cast_slice(VERTICES), context, "Vertex buffer");
        buffers.create_index_buffer_init(bytemuck::cast_slice(INDICES), context, "Index buffer");
        buffers.create_uniform_buffer(context, "Camera buffer", std::mem::size_of::<CameraUniform>() as u64);
        buffers.create_uniform_buffer(context, "Frame data buffer", 16);

        let binding_0 = BindGroupContainer::create_layout(0, context, "Frame data bind group");
        let binding_1 = BindGroupContainer::create_layout(0, context, "Camera bind group");

        bind_group_layouts = [&binding_0, &binding_1];
        bind_groups.create_bind_group(
            0,
            &buffers.get("Frame data buffer"),
            context,
            "Frame data bind group",
            bind_group_layouts[0],
        );
        bind_groups.create_bind_group(
            0,
            &buffers.get("Camera buffer"),
            context,
            "Camera bind group",
            bind_group_layouts[1],
        );

        let mut pipeline_builder = PiplineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(context.surface_config.format);
        let render_pipeline = pipeline_builder.build(&context.device, &bind_group_layouts);

        Self {
            bind_groups,
            buffers,
            render_pipeline,
        }
    }
    pub fn render(
        &self,
        camera: &Camera,
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
            label: Some("Main render Pass"),
            color_attachments: &[Some(color_attachment)],
            ..Default::default()
        };

        let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);

        let frame_data: [f32; 2] = [1920.0, 1080.0];

        context
            .queue
            .write_buffer(&self.buffers.get("Frame data buffer"), 0, bytemuck::cast_slice(&frame_data));

        context
            .queue
            .write_buffer(&self.buffers.get("Camera buffer"), 0, bytemuck::cast_slice(&[camera.get_uniform()]));

        render_pass.set_bind_group(0, &self.bind_groups.get("Frame data bind group"), &[]);
        render_pass.set_bind_group(1, &self.bind_groups.get("Camera bind group"), &[]);
        render_pass.set_vertex_buffer(0, self.buffers.get("Vertex buffer").slice(..));
        render_pass.set_index_buffer(self.buffers.get("Index buffer").slice(..), wgpu::IndexFormat::Uint16);

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        drop(render_pass);

        egui.draw(context, &drawable, &mut command_encoder, |ui| gui(ui, frametime), window);

        context.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }
}
