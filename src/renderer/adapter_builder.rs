pub struct AdapterBuilder;

impl AdapterBuilder {
    pub fn new() -> Self {
        Self
    }

    pub async fn build(&self, instance: &wgpu::Instance, surface: &wgpu::Surface<'_>) -> wgpu::Adapter {
        let adapter_descriptor = wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();
        adapter
    }
}