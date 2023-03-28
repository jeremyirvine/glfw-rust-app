use enum_iterator::all;
use imgui_glfw_rs::imgui::Ui;

use crate::{renderer::Renderer, str_to_imstr};

use super::{
    test_batch_rendering::TestBatchRendering, test_clear_color::TestClearColor,
    test_texture::TestTexture, TestType, TestTypeInternal, Testable,
};

#[derive(Default)]
pub struct TestMenu {
    active_test: Option<TestTypeInternal>,
}

impl TestMenu {
    pub fn render(&self, screen_size: (f32, f32), renderer: &Renderer) {
        if let Some(active_test) = &self.active_test {
            active_test.render(screen_size, renderer);
        }
    }

    pub fn imgui_render(&mut self, screen_size: (f32, f32), ui: &Ui) {
        if let Some(active_test) = &mut self.active_test {
            if ui.button(&str_to_imstr("<-- Back"), [150., 20.]) {
                self.active_test = None;
            } else {
                active_test.imgui_render(screen_size, ui);
            }
        } else {
            for test in all::<TestType>() {
                if ui.button(&str_to_imstr(test.clone()), [150., 20.]) {
                    self.use_test(test);
                }
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if let Some(active_test) = &mut self.active_test {
            active_test.update(delta_time);
        }
    }

    pub fn imgui_title(&self) -> String {
        match &self.active_test {
            Some(active_test) => format!("Test `{}`", active_test.test_name()),
            None => "Test Menu".into(),
        }
    }

    pub fn use_test(&mut self, test: TestType) {
        self.active_test = Some(match test {
            TestType::ClearColor => TestTypeInternal::ClearColor(TestClearColor::default()),
            TestType::Texture => TestTypeInternal::Texture(TestTexture::default()),
            TestType::BatchRendering => {
                TestTypeInternal::BatchRendering(TestBatchRendering::default())
            }
        });
    }
}
