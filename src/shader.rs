#![allow(dead_code,unused)]

use std::{ffi::CString, ptr};
use gl::types::{GLchar, GLint};

use crate::gl_component::GLComponent;

pub enum ShaderType {
    None,
    Vertex,
    Fragment
}

#[derive(Default)]
pub struct ShaderBuilder {
    fragment_src: Option<String>,
    vertex_src: Option<String>,
}

impl ShaderBuilder {
    pub fn with_shader_source(mut self, source: String) -> Result<Self, ::std::io::Error> {
        let mut vert_src = String::new();
        let mut frag_src = String::new();
        let mut append_to_type = ShaderType::None;
        for line in source.lines() {
            if line.contains("#shader") {
                if line.contains("vertex") {
                    append_to_type = ShaderType::Vertex;
                    self.vertex_src = Some(String::new());
                }
                else if line.contains("fragment") {
                    append_to_type = ShaderType::Fragment;
                    self.fragment_src = Some(String::new());
                }
            } else {
                let line = format!("{}\n", line);
                let line = line.as_str();
                match append_to_type {
                    ShaderType::Vertex => vert_src.push_str(line.clone()),
                    ShaderType::Fragment => frag_src.push_str(line),
                    _ => {},
                }
            }
        }

        Ok(self.with_fragment(frag_src)?.with_vertex(vert_src)?)
    }

    pub fn with_shader(self, path: String) -> Result<Self, ::std::io::Error> {
        let shader_source_bin = ::std::fs::read(path)?;
        let shader_source = String::from_utf8_lossy(&shader_source_bin).to_string();
        self.with_shader_source(shader_source)
    }

    fn with_fragment(mut self, src: String) -> Result<Self, ::std::io::Error> {
        self.fragment_src = Some(src);
        Ok(self)
    }

    fn with_vertex(mut self, src: String) -> Result<Self, ::std::io::Error> {
        self.vertex_src = Some(src);
        Ok(self)
    }

    pub fn build(&self) -> Shader {
       if let (Some(frag_src), Some(vert_src)) = (self.fragment_src.clone(), self.vertex_src.clone()) {  
            Shader::from_sources(frag_src, vert_src)
       } else {
            panic!("");
       }
    }
}

pub struct Shader {
    renderer_id: u32,
}

impl GLComponent for Shader {
    fn renderer_id(&self) -> u32 {
        self.renderer_id
    }

    fn bind(&self) {
        unsafe { gl::UseProgram(self.renderer_id) }
    }

    fn unbind(&self) {
        unsafe { gl::UseProgram(0) }
    }
}

impl Shader {
    fn uniform_location(&self, location: String) -> GLint {
        let cname = ::std::ffi::CString::new(location).expect("Failed to convert uniform location to CString");
        unsafe { gl::GetUniformLocation(self.renderer_id, cname.as_ptr()) }
    } 

    pub fn uniform_4f(&self, location: impl Into<String>, val: (f32, f32, f32, f32)) {
        let (v0,v1,v2,v3) = val;
        let location = self.uniform_location(location.into());
        unsafe { gl::Uniform4f(location, v0, v1, v2, v3) }
    }

    pub fn uniform_3f(&self, location: impl Into<String>, val: (f32, f32, f32)) {
        let (v0,v1,v2) = val;
        let location = self.uniform_location(location.into());
        unsafe { gl::Uniform3f(location, v0, v1, v2) }
    }

    pub fn uniform_2f(&self, location: impl Into<String>, val: (f32, f32)) {
        let (v0,v1) = val;
        let location = self.uniform_location(location.into());
        unsafe { gl::Uniform2f(location, v0, v1) }
    }

    pub fn uniform_1f(&self, location: impl Into<String>, val: f32) {
        let location = self.uniform_location(location.into());
        unsafe { gl::Uniform1f(location, val) }
    }

    pub fn from_sources(fragment_src: String, vertex_src: String) -> Self {
        let fragment_id = {
            let shader_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
            let c_str_vert = CString::new(fragment_src).unwrap();
            unsafe { gl::ShaderSource(shader_id, 1, &c_str_vert.as_ptr(), ptr::null()) };
            unsafe { gl::CompileShader(shader_id) };
            
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            unsafe { info_log.set_len(512 - 1); }
            unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success); };
            if success != gl::TRUE as GLint {
                unsafe { gl::GetShaderInfoLog(shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar) };
                println!("[Shader Error] [Fragment] Compilation Failed\n{}", std::str::from_utf8(&info_log).unwrap());
            }
            shader_id
        };

        let vertex_id = {
            let shader_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
            let c_str_vert = CString::new(vertex_src).unwrap();
            unsafe { gl::ShaderSource(shader_id, 1, &c_str_vert.as_ptr(), ptr::null()) };
            unsafe { gl::CompileShader(shader_id) };
            
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            unsafe { info_log.set_len(512 - 1); }
            unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success) };
            if success != gl::TRUE as GLint {
                unsafe { gl::GetShaderInfoLog(shader_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar) };
                println!("[Shader Error] [Vertex] Compilation Failed\n{}", std::str::from_utf8(&info_log).unwrap());
            }
            shader_id
        };

        let renderer_id = {
            let renderer_id = unsafe { gl::CreateProgram() };
            unsafe {
                gl::AttachShader(renderer_id, vertex_id);
                gl::AttachShader(renderer_id, fragment_id);
                gl::LinkProgram(renderer_id);
            }
            
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            unsafe { info_log.set_len(512 - 1); }
            unsafe { gl::GetProgramiv(renderer_id, gl::LINK_STATUS, &mut success) }
            if success != gl::TRUE as GLint {
                unsafe { gl::GetProgramInfoLog(renderer_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar); }
                println!("[Shader Error] [Program] Linker Failed\n{}", std::str::from_utf8(&info_log).unwrap());
            }
            
            unsafe { gl::DeleteShader(vertex_id) };
            unsafe { gl::DeleteShader(fragment_id) };
            renderer_id
        };

        Self {
            renderer_id,
        }
    }
}
