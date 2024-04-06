pub struct InstanceBuilder;

impl InstanceBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self) -> wgpu::Instance {
        let descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(descriptor);

        instance
    }
}