#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            position: [0.0; 3],
            color: [1.0; 3],
        }
    }

    pub fn set_vertex(&mut self, position: [f32; 3], color: [f32; 3]) {
        self.position = position;
        self.color = color;
    }
}
