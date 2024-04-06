pub struct InstanceBuilder;

impl InstanceBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self) -> wgpu::Instance {
        let descriptor = wgpu::InstanceDescriptor::default();
        wgpu::Instance::new(descriptor)
    }
}
