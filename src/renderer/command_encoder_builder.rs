pub struct CommandEncoderBuilder;

impl CommandEncoderBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self, device: &wgpu::Device) -> wgpu::CommandEncoder {
        let descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        device.create_command_encoder(&descriptor)
    }
}
