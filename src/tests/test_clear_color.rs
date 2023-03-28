use glcall_macro::gl_call;
use imgui_glfw_rs::imgui::Ui;

use crate::{renderer::Renderer, str_to_imstr};

use super::{Testable, TestableID};

pub struct TestClearColor {
    color: [f32; 4],
}

impl Testable for TestClearColor {
    fn render(&self, _: (f32, f32), _: &Renderer) {
        let (red, green, blue, alpha) =
            (self.color[0], self.color[1], self.color[2], self.color[2]);
        gl_call!({
            gl::ClearColor(red, green, blue, alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        });
    }

    fn imgui_render(&mut self, _: (f32, f32), ui: &Ui) {
        ui.color_picker(&str_to_imstr("Clear Color"), &mut self.color)
            .build();
    }

    fn update(&mut self, _delta_time: f32) {}
}

impl TestableID for TestClearColor {
    fn test_id() -> String {
        "clear_color_test".into()
    }

    fn test_name() -> String {
        "Clear Color".into()
    }
}

impl Default for TestClearColor {
    fn default() -> Self {
        Self {
            color: [0.2, 0.3, 0.8, 1.0],
        }
    }
}
