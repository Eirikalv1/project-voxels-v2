use glam::{vec3, Mat4, Quat, Vec3};

use crate::input::InputState;

pub struct Camera {
    direction: Vec3,
    position: Vec3,
    projection_inverse: Mat4,
    uniform: CameraUniform,
    view_inverse: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            direction: vec3(0.0, 0.0, -1.0),
            position: vec3(0.0, 0.0, 3.0),
            projection_inverse: Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 1.0, 100.0).inverse(),
            uniform: CameraUniform::new(),
            view_inverse: Mat4::IDENTITY,
        }
    }

    pub fn on_update(&mut self, input: &InputState) {
        let up_direction = Vec3::Y;
        let right_direction = self.direction.cross(up_direction);

        let translation_speed = 0.05;

        if input.w {
            self.position += translation_speed * self.direction;
        } else if input.s {
            self.position -= translation_speed * self.direction;
        }
        if input.a {
            self.position -= translation_speed * right_direction;
        } else if input.d {
            self.position += translation_speed * right_direction;
        }
        if input.q {
            self.position -= translation_speed * up_direction;
        } else if input.e {
            self.position += translation_speed * up_direction;
        }

        let rotation_speed = 1.0;

        let mut pitch_delta = 0.0;
        let mut yaw_delta = 0.0;

        if input.right_mouse_button {
            pitch_delta = input.delta_mouse_position.y * rotation_speed;
            yaw_delta = input.delta_mouse_position.x * rotation_speed;
        }

        let rotation = Quat::from_axis_angle(right_direction, -pitch_delta) * Quat::from_axis_angle(up_direction, -yaw_delta).normalize();

        self.direction = rotation * self.direction;

        self.view_inverse = Mat4::look_to_rh(self.position, self.direction, up_direction).inverse();

        self.uniform
            .update_projections(self.position, self.projection_inverse, self.view_inverse);
    }

    pub fn get_uniform(&self) -> CameraUniform {
        self.uniform
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    position: [f32; 3],
    _padding1: f32,
    projection_inverse: [[f32; 4]; 4],
    view_inverse: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 3.0],
            _padding1: 0.0,
            projection_inverse: Mat4::IDENTITY.to_cols_array_2d(),
            view_inverse: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_projections(&mut self, position: Vec3, projection_inverse: Mat4, view_inverse: Mat4) {
        self.position = position.into();
        self.projection_inverse = projection_inverse.to_cols_array_2d();
        self.view_inverse = view_inverse.to_cols_array_2d();
    }
}
