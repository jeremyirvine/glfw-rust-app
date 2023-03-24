pub mod gl_error;
pub use gl_error::{gl_clear_errors, gl_log_errors};

pub mod gl_component;

pub mod index_buffer;
pub mod vertex_array;
pub mod vertex_buffer;
pub mod vertex_buffer_layout;

pub mod renderer;
pub mod texture;

pub mod shader;
pub use shader::ShaderBuilder;


use imgui_glfw_rs::imgui::ImString;
pub fn str_to_imstr(s: impl Into<String>) -> ImString {
    let s: String = s.into();
    ImString::from(s)
}
