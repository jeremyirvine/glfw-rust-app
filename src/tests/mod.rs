use imgui_glfw_rs::imgui::Ui;

use crate::renderer::Renderer;

pub trait Testable {
    fn render(&self, screen_size: (f32, f32), renderer: &Renderer);
    fn imgui_render(&mut self, screen_size: (f32, f32),ui: &Ui);
    fn update(&mut self, delta_time: f32);

    fn test_id(&self) -> &str;
    fn test_name(&self) -> &str;
}

pub mod menu;
pub mod test_clear_color;
pub mod test_texture;
pub mod test_batch_rendering;
