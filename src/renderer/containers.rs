use std::collections::HashMap;

use wgpu::util::DeviceExt;

use crate::GpuContext;

pub struct BindGroupContainer(HashMap<String, wgpu::BindGroup>);

impl BindGroupContainer {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, name: &str) -> &wgpu::BindGroup {
        if let Some(bind_group) = self.0.get(name) {
            return bind_group;
        }
        log::error!("Bind group name not recognized: {}.", name);
        panic!();
    }

    pub fn create_layout(binding: u32, context: &GpuContext, label: &str) -> wgpu::BindGroupLayout {
        context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some(label),
        })
    }
    pub fn create_bind_group(
        &mut self,
        binding: u32,
        content: &wgpu::Buffer,
        context: &GpuContext,
        label: &str,
        layout: &wgpu::BindGroupLayout,
    ) {
        let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding,
                resource: content.as_entire_binding(),
            }],
            label: Some(label),
        });
        self.0.insert(label.to_string(), bind_group);
    }
}

pub struct BufferContainer(HashMap<String, wgpu::Buffer>);

impl BufferContainer {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, name: &str) -> &wgpu::Buffer {
        if let Some(buffer) = self.0.get(name) {
            return buffer;
        }
        log::error!("Buffer name not recognized: {}.", name);
        panic!();
    }

    pub fn create_index_buffer_init(&mut self, contents: &[u8], context: &GpuContext, label: &str) {
        let buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents,
            usage: wgpu::BufferUsages::INDEX,
        });
        self.0.insert(label.to_string(), buffer);
    }
    pub fn create_uniform_buffer(&mut self, context: &GpuContext, label: &str, size: u64) {
        let buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
            label: Some(label),
        });
        self.0.insert(label.to_string(), buffer);
    }
    pub fn create_uniform_buffer_init(&mut self, contents: &[u8], context: &GpuContext, label: &str) {
        let buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        self.0.insert(label.to_string(), buffer);
    }
    pub fn create_vertex_buffer_init(&mut self, contents: &[u8], context: &GpuContext, label: &str) {
        let buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents,
            usage: wgpu::BufferUsages::VERTEX,
        });
        self.0.insert(label.to_string(), buffer);
    }
}
