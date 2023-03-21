use std::ffi::c_void;

use glcall_macro::gl_call;

use crate::{vertex_array::VertexArray, shader::Shader, index_buffer::IndexBuffer, gl_component::GLComponent};
use crate::{gl_clear_errors, gl_log_errors};

pub struct Renderer {
    clear_color: (f32, f32, f32, f32),
}
impl Renderer {
    pub fn new(clear_color: (f32, f32, f32, f32)) -> Self {
        Self {
            clear_color
        }
    }

    pub fn clear(&self) {
        let (r,g,b,a) = self.clear_color;
        gl_call!({
            gl::ClearColor(r,g,b,a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        });
    }

    pub fn draw(&self, va: &VertexArray, ib: &IndexBuffer, shader: &Shader) {
        shader.bind();
        va.bind();
        ib.bind();
        gl_call!({ gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void); });
    }
}