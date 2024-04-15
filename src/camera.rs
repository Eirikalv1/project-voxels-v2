use glam::{vec3, Mat4, Quat, Vec3};

use crate::input::InputState;

pub struct Camera {
    position: Vec3,
    direction: Vec3,
    view_inverse: Mat4,
    projection_inverse: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 0.0, 3.0),
            direction: vec3(0.0, 0.0, -1.0),
            view_inverse: Mat4::IDENTITY,
            projection_inverse: Mat4::perspective_rh(std::f32::consts::PI / 4.0, 1.0, 0.1, 100.0).inverse(),
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

        let rotation_speed = 0.1;

        let mut pitch_delta = 0.0;
        let mut yaw_delta = 0.0;
        if input.right_mouse_button {
            pitch_delta = input.delta_mouse_position.y * rotation_speed;
            yaw_delta = input.delta_mouse_position.x * rotation_speed;
        }

        let rotation = Quat::from_axis_angle(right_direction, -pitch_delta) * Quat::from_axis_angle(up_direction, -yaw_delta).normalize();

        self.direction = rotation * self.direction;

        self.view_inverse = Mat4::look_to_rh(self.position, self.direction, Vec3::Y).inverse();
    }
}
