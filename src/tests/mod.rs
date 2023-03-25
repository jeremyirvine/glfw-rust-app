use imgui_glfw_rs::imgui::Ui;

pub trait Testable {
    fn render(&self);
    fn imgui_render(&mut self, ui: &Ui);
    fn update(&mut self, delta_time: f32);

    fn test_id(&self) -> &str;
    fn test_name(&self) -> &str;
}

pub mod menu;
pub mod test_clear_color;
