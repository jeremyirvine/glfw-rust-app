use std::ffi::c_void;

use gl::types::GLuint;
use glcall_macro::gl_call;
use nalgebra_glm as glm;

use crate::{tests::Testable, shader::Shader, ShaderBuilder, vertex_array::VertexArray, vertex_buffer_layout::VertexBufferLayout, vertex_buffer::VertexBuffer, index_buffer::IndexBuffer, gl_component::GLComponent, texture::Texture, str_to_imstr};

fn gen_square(size: f32, x: f32, y: f32, color: Vec<f32>) -> Vec<f32> {
    let half = size / 2.;

    #[rustfmt::skip]
    let vertices = vec![
        (-half) + x,    half + y, 0.0,     color[0], color[1], color[2],
           half + x,    half + y, 0.0,     color[0], color[1], color[2],
           half + x, (-half) + y, 0.0,     color[0], color[1], color[2],
        (-half) + x, (-half) + y, 0.0,     color[0], color[1], color[2],
    ];

    vertices
}

pub struct TestBatchRendering {
    vao: u32,
    vbo: u32,
    ibo: u32,
    shader: Shader,

    model: glm::Vec3,
}

impl Default for TestBatchRendering {
    fn default() -> Self {
        let square_1 = gen_square(100., 0., 0.0,   vec![0.18, 0.6, 0.96]);
        let square_2 = gen_square(100., 200., 0.0, vec![1.0, 0.96, 0.24]);

        let mut vertices: Vec<f32> = square_1;
        vertices.extend(square_2);


        #[rustfmt::skip]
        let indices: Vec<GLuint> = vec![
            0,1,2,2,3,0,

            4,5,6,6,7,4
        ];
        
        let shader = ShaderBuilder::default()
            .with_shader_source(include_str!("../res/shaders/Batching.glsl").into())
            .expect("Failed to build shader from source")
            .build();
        
        let mut vao = 0;
        let mut vbo = 0;
        let mut ibo = 0;

        gl_call!({
           gl::GenVertexArrays(1, &mut vao);
           gl::BindVertexArray(vao);
        });

        gl_call!({
            gl::CreateBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                (vertices.len() * std::mem::size_of::<f32>()) as isize, 
                &vertices[0] as *const f32 as *const c_void, 
                gl::STATIC_DRAW,
            );
            
            // Vertices
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, 
                3, 
                gl::FLOAT, 
                gl::FALSE, 
                (6 * std::mem::size_of::<f32>()) as i32, 
                std::ptr::null(),
            );

            // Color
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, 
                3, 
                gl::FLOAT, 
                gl::FALSE, 
                (6 * std::mem::size_of::<f32>()) as i32, 
                12 as *const c_void,
            );
        });

        gl_call!({
            gl::CreateBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                (indices.len() * std::mem::size_of::<GLuint>()) as isize, 
                &indices[0] as *const GLuint as *const c_void, 
                gl::STATIC_DRAW,
            );
        });


        Self { shader, vao, vbo, ibo, model: glm::vec3(200., 200., 0.) }
    }
}

impl Testable for TestBatchRendering {
    fn render(&self, (width, height): (f32, f32), _renderer: &crate::renderer::Renderer) {
        let proj = glm::ortho(
            0.0,
            width,
            0.0,
            height,
            -1.0,
            1.0,
        );
        let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, 0.0));
        let model = glm::translate(&glm::Mat4::identity(), &self.model);

        let mvp = proj * view * model;

        self.shader.bind();
        self.shader.uniform_mat4("u_MVP", &mvp);
        gl_call!({
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, std::ptr::null());
        });
    }

    fn imgui_render(&mut self, _: (f32, f32), ui: &imgui_glfw_rs::imgui::Ui) {
        let model = self.model.as_mut_slice();
        ui.slider_float3(&str_to_imstr("Model Transform"), model.try_into().unwrap(), 0.0, 600.).build();
    }

    fn update(&mut self, _delta_time: f32) {
    }

    fn test_id(&self) -> &str {
        "batch_rendering"
    }

    fn test_name(&self) -> &str {
        "Batch Rendering"
    }
}
