use winit::window::Window;

pub struct SurfaceBuilder {

}

impl SurfaceBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build<'a>(&self, instance: &wgpu::Instance, window: &'a Window) -> wgpu::Surface<'a> {
        let surface = instance.create_surface(window).unwrap();
        surface
    }
}

