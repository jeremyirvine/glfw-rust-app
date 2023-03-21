use std::ffi::c_void;
use glcall_macro::gl_call;
use crate::{gl_component::GLComponent, gl_error::{gl_clear_errors, gl_log_errors}};

pub struct VertexBuffer {
    renderer_id: u32,
}

impl GLComponent for VertexBuffer {
    fn renderer_id(&self) -> u32 {
        self.renderer_id
    }

    fn bind(&self) {
        gl_call!({ gl::BindBuffer(gl::ARRAY_BUFFER, self.renderer_id); });
    }

    fn unbind(&self) {
        gl_call!({ gl::BindBuffer(gl::ARRAY_BUFFER, 0); });
    }
}

impl VertexBuffer {
    pub fn new<T>(data: &Vec<T>) -> Self {
        let mut renderer_id = 0;
        gl_call!({
            gl::GenBuffers(1, &mut renderer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, renderer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (data.len() * std::mem::size_of::<T>()) as isize,
                &data[0] as *const T as *const c_void,
                gl::STATIC_DRAW
            );
        });
        Self {
            renderer_id,
        }
    }
}