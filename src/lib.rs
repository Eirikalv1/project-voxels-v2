use std::time::Instant;

use winit::{dpi::PhysicalSize, window::Window};

pub mod engine_loop;
mod renderer;

pub struct GpuContext<'a> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
}

impl<'a> GpuContext<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let instance_descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(instance_descriptor);

        let surface = instance.create_surface(window).expect("Failed to create surface.");

        let adapter_descriptor = wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        };
        let adapter = instance
            .request_adapter(&adapter_descriptor)
            .await
            .expect("Failed to request adapter.");

        let device_descriptor = wgpu::DeviceDescriptor {
            label: Some("Device"),
            ..Default::default()
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await
            .expect("Failed to request device.");

        let window_size = window.inner_size();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        Self {
            device,
            queue,
            surface,
            surface_config,
        }
    }

    pub fn resize_surface_config(&mut self, new_size: &PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
}

pub struct FrameTimer {
    start: Instant,
    end: Instant,
}

impl FrameTimer {
    pub fn new() -> Self {
        FrameTimer {
            start: Instant::now(),
            end: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn end(&mut self) {
        self.end = Instant::now();
    }

    pub fn delta_time(&self) -> u128 {
        (self.end - self.start).as_millis()
    }
}
