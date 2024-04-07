use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::{egui::Context, State};
use wgpu::SurfaceTexture;
use winit::{event::WindowEvent, window::Window};

use crate::GpuContext;

// From: https://github.com/ejb004/egui-wgpu-demo/blob/master/src/gui.rs

pub struct EguiRenderer {
    context: Context,
    state: State,
    renderer: Renderer,
}

impl EguiRenderer {
    pub fn new(device: &wgpu::Device, window: &Window, surface_format: wgpu::TextureFormat) -> Self {
        let context = Context::default();
        let id = context.viewport_id();
        let state = State::new(context.clone(), id, window, None, None);
        let renderer = Renderer::new(device, surface_format, None, 1);

        Self { context, state, renderer }
    }

    pub fn draw(
        &mut self,
        context: &GpuContext,
        drawable: &SurfaceTexture,
        encoder: &mut wgpu::CommandEncoder,
        run_ui: impl FnOnce(&Context),
        window: &Window,
    ) {
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [context.surface_config.width, context.surface_config.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        let window_surface_view = drawable.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Egui texture"),
            format: None,
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        let raw_input = self.state.take_egui_input(window);
        let full_output = self.context.run(raw_input, |_| {
            run_ui(&self.context);
        });

        self.state.handle_platform_output(window, full_output.platform_output);

        let tris = self.context.tessellate(full_output.shapes, full_output.pixels_per_point);
        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer.update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer
            .update_buffers(&context.device, &context.queue, encoder, &tris, &screen_descriptor);
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &window_surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            label: Some("Egui render pass"),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        self.renderer.render(&mut render_pass, &tris, &screen_descriptor);
        drop(render_pass);
        for x in &full_output.textures_delta.free {
            self.renderer.free_texture(x)
        }
    }

    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }
}

pub fn gui(ui: &Context, frametime: u128) {
    egui::Window::new("Egui")
        .default_open(true)
        .max_width(1000.0)
        .max_height(800.0)
        .default_width(800.0)
        .resizable(true)
        .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
        .show(ui, |ui| {
            ui.label(format!("Frametime: {}ms", frametime));

            ui.end_row();
        });
}
