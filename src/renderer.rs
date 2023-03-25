use std::ffi::c_void;

use glcall_macro::gl_call;
use nalgebra_glm::Vec4;

use crate::{
    gl_component::GLComponent, index_buffer::IndexBuffer, shader::Shader, vertex_array::VertexArray,
};

pub struct Renderer {
    clear_color: (f32, f32, f32, f32),
}

impl From<Vec4> for Renderer {
    fn from(value: Vec4) -> Self {
        let color = value.as_slice();
        assert!(
            color.len() >= 4,
            "impl From<Vec4> for Renderer requires a vector of at least 4 elements"
        );
        Self::new((color[0], color[1], color[2], color[3]))
    }
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
            gl::DrawElements(
                gl::TRIANGLES,
                6,
                gl::UNSIGNED_INT,
                std::ptr::null::<c_void>(),
            );
        });
    }

    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.clear_color = (r, g, b, a);
    }

    pub fn set_clear_color_from_vec4(&mut self, vector: &Vec4) {
        let color = vector.as_slice();
        assert!(
            color.len() >= 4,
            "set_clear_color_from_vec4(..) requires a vector of at least 4 elements"
        );
        self.set_clear_color(color[0], color[1], color[2], color[3]);
    }
}
