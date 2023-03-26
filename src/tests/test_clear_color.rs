use glcall_macro::gl_call;
use imgui_glfw_rs::imgui::Ui;

use crate::{str_to_imstr, renderer::Renderer};

use super::Testable;

pub struct TestClearColor {
    color: [f32; 4],
}

impl Testable for TestClearColor {
    fn render(&self, _: (f32, f32), renderer: &Renderer) {
        let (red, green, blue, alpha) =
            (self.color[0], self.color[1], self.color[2], self.color[2]);
        gl_call!({
            gl::ClearColor(red, green, blue, alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        });
    }

    fn imgui_render(&mut self, screen_size: (f32,f32), ui: &Ui) {
        ui.color_picker(&str_to_imstr("Clear Color"), &mut self.color)
            .build();
    }

    fn update(&mut self, _delta_time: f32) {}

    fn test_id(&self) -> &str {
        "clear_color_test"
    }

    fn test_name(&self) -> &str {
        "Clear Color"
    }
}

impl Default for TestClearColor {
    fn default() -> Self {
        Self {
            color: [0.2, 0.3, 0.8, 1.0],
        }
    }
}
