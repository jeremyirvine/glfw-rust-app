use std::ffi::c_void;

use glcall_macro::gl_call;

use crate::gl_component::GLComponent;
use crate::gl_error::{gl_clear_errors, gl_log_errors};
use crate::vertex_buffer::VertexBuffer;
use crate::vertex_buffer_layout::{VertexBufferLayout, size_of_type};

pub struct VertexArray {
    renderer_id: u32,
}

impl GLComponent for VertexArray {
    fn renderer_id(&self) -> u32 {
        self.renderer_id
    }

    fn bind(&self) {
        gl_call!({ gl::BindVertexArray(self.renderer_id); });
    }

    fn unbind(&self) {
        gl_call!({ gl::BindVertexArray(0); });
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        gl_call!({ gl::DeleteVertexArrays(1, &mut self.renderer_id); });
    }
}

impl VertexArray {
    pub fn new() -> Self {
        let mut renderer_id = 0u32;
        gl_call!({ 
            gl::GenVertexArrays(1, &mut renderer_id);
            gl::BindVertexArray(renderer_id);
        });
        Self { renderer_id }
    }

    pub fn add_buffer(&mut self, buffer: &VertexBuffer, layout: &VertexBufferLayout) {
        self.bind();
        buffer.bind();
        let elements = layout.elements();
        let mut offset = 0;
        for (i, element) in elements.iter().enumerate() {
            gl_call!({
                gl::EnableVertexAttribArray(i as u32);
                gl::VertexAttribPointer(
                    i as u32, 
                    element.count() as i32, 
                    element.gl_type(), 
                    if element.normalized() { gl::TRUE } else { gl::FALSE }, 
                    layout.stride() as i32, 
                    offset as *const c_void
                );
            });
            offset += element.count() * size_of_type(element.gl_type());

        }
    }
}