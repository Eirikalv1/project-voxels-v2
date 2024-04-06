pub struct DeviceBuilder;

impl DeviceBuilder {
    pub fn new() -> Self {
        Self
    }

    pub async fn build(&self, adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        let device_descriptor = wgpu::DeviceDescriptor {
            label: Some("Device"),
            ..Default::default()
        };
        let (device, queue) = adapter.request_device(&device_descriptor, None).await.unwrap();
        (device, queue)
    }
}