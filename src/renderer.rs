use std::ffi::c_void;

use glcall_macro::gl_call;

use crate::{
    gl_component::GLComponent, index_buffer::IndexBuffer, shader::Shader, vertex_array::VertexArray,
};

pub struct Renderer {
    clear_color: (f32, f32, f32, f32),
}
impl Renderer {
    pub fn new(clear_color: (f32, f32, f32, f32)) -> Self {
        Self { clear_color }
    }

    pub fn clear(&self) {
        let (r, g, b, a) = self.clear_color;
        gl_call!({
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        });
    }

    pub fn draw(&self, va: &VertexArray, ib: &IndexBuffer, shader: &Shader) {
        shader.bind();
        va.bind();
        ib.bind();
        gl_call!({
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
        });
    }
}
