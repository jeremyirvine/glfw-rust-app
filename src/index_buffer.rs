use std::ffi::c_void;
use glcall_macro::gl_call;
use crate::{gl_component::GLComponent, gl_error::{gl_clear_errors, gl_log_errors}};

pub struct IndexBuffer {
    renderer_id: u32,
}

impl GLComponent for IndexBuffer {
    fn renderer_id(&self) -> u32 {
        self.renderer_id
    }

    fn bind(&self) {
        gl_call!({ gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.renderer_id); });
    }

    fn unbind(&self) {
        gl_call!({ gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); });
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        gl_call!({ gl::DeleteBuffers(1, &mut self.renderer_id); });
    }
}

impl IndexBuffer {
    pub fn new(data: &Vec<u32>) -> Self {
        let mut renderer_id = 0;
        gl_call!({
            gl::GenBuffers(1, &mut renderer_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, renderer_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                (data.len() * std::mem::size_of::<u32>()) as isize,
                &data[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW
            );
        });
        Self {
            renderer_id,
        }
    }
}