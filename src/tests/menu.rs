use imgui_glfw_rs::imgui::Ui;

use crate::str_to_imstr;

use super::Testable;

#[derive(Default)]
pub struct TestMenu {
    /// List of trait objects that are registered as tests
    tests: Vec<Box<dyn Testable>>,
    
    /// The index of the currently running test (indexes into self.tests)
    current_test: Option<usize>,
}

impl TestMenu {
    pub fn register_test(&mut self, test: Box<dyn Testable>) {
        self.tests.push(test); 
    }

    pub fn render(&self) {
        if let Some(current_test) = self.current_test {
            self.tests[current_test].render();
        } else {
            
        }
    }

    pub fn imgui_render(&mut self, ui: &Ui) {
        if let Some(current_test) = self.current_test {
            if ui.button(&str_to_imstr("<-- Back"), [100., 20.]) {
                self.current_test = None; 
            } else {
                self.tests[current_test].imgui_render(ui);
            }
        } else {
            for (i, test) in (&self.tests).iter().enumerate() {
                if ui.button(&str_to_imstr(test.test_name()), [100., 20.]) {
                    self.current_test = Some(i);
                }
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if let Some(current_test) = self.current_test {
            self.tests[current_test].update(delta_time);
        } else {

        }
    }

    pub fn imgui_title(&self) -> String {
        match self.current_test {
            Some(current_test) => format!("Test `{}`", self.tests[current_test].test_name()),
            None => "Test Menu".into(),
        }
    }

    pub fn use_test(&mut self, test: usize) {
        if self.tests.len() > test {
            self.current_test = Some(test);
        }
    }
}
