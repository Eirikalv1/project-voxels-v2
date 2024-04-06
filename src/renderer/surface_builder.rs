use wgpu::SurfaceCapabilities;
use winit::window::Window;

pub struct SurfaceBuilder;

impl SurfaceBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build<'a>(&self, instance: &wgpu::Instance, window: &'a Window) -> wgpu::Surface<'a> {
        instance.create_surface(window).unwrap()
    }

    pub fn create_initial_format(&self, capabilities: &SurfaceCapabilities) -> wgpu::TextureFormat {
        capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(capabilities.formats[0])
    }

    pub fn create_initial_configuration(
        &self,
        capabilities: &SurfaceCapabilities,
        format: &wgpu::TextureFormat,
        window: &Window,
    ) -> wgpu::SurfaceConfiguration {
        let size = window.inner_size();
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }
}
