use std::ffi::c_void;

use gl::types::GLuint;
use glcall_macro::gl_call;
use nalgebra_glm as glm;

use crate::{
    gl_component::GLComponent, shader::Shader, str_to_imstr, tests::Testable, texture::Texture,
    ShaderBuilder,
};

use super::TestableID;

pub struct TestBatchRendering {
    vao: u32,
    shader: Shader,

    model: glm::Vec3,

    phone_texture: Texture,
    rust_texture: Texture,
}

impl Default for TestBatchRendering {
    fn default() -> Self {
        #[rustfmt::skip]
        let vertices: Vec<f32> = vec![
            // Quad 1
            -50.,  50., 0.0,        0.18, 0.6, 0.96, 1.0,    0.0, 1.0,    0.0,
             50.,  50., 0.0,        0.18, 0.6, 0.96, 1.0,    1.0, 1.0,    0.0,
             50., -50., 0.0,        0.18, 0.6, 0.96, 1.0,    1.0, 0.0,    0.0,
            -50., -50., 0.0,        0.18, 0.6, 0.96, 1.0,    0.0, 0.0,    0.0,
                    
            // Quad 2
             100.,  50., 0.0,       1.0, 0.96, 0.24, 1.0,    0.0, 1.0,    1.0,
             200.,  50., 0.0,       1.0, 0.96, 0.24, 1.0,    1.0, 1.0,    1.0,
             200., -50., 0.0,       1.0, 0.96, 0.24, 1.0,    1.0, 0.0,    1.0,
             100., -50., 0.0,       1.0, 0.96, 0.24, 1.0,    0.0, 0.0,    1.0,
        ];

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
                (10 * std::mem::size_of::<f32>()) as i32,
                std::ptr::null(),
            );

            // Color
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                (10 * std::mem::size_of::<f32>()) as i32,
                12 as *const c_void,
            );

            // Texture Coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (10 * std::mem::size_of::<f32>()) as i32,
                28 as *const c_void,
            );

            // Texture Index
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                1,
                gl::FLOAT,
                gl::FALSE,
                (10 * std::mem::size_of::<f32>()) as i32,
                36 as *const c_void,
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

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        });

        let phone_texture = Texture::new("src/res/textures/phone.png", 0);
        let rust_texture = Texture::new("src/res/textures/rust.png", 1);

        shader.bind();
        //shader.uniform_vec_int("u_Textures", &vec![0, 1]);
        shader.uniform_1i("u_Texture0", 0);
        shader.uniform_1i("u_Texture1", 1);
        shader.unbind();

        Self {
            shader,
            vao,
            model: glm::vec3(200., 200., 0.),
            phone_texture,
            rust_texture,
        }
    }
}

impl Testable for TestBatchRendering {
    fn render(&self, (width, height): (f32, f32), _renderer: &crate::renderer::Renderer) {
        gl_call!({
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        });
        let proj = glm::ortho(0.0, width, 0.0, height, -1.0, 1.0);
        let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, 0.0));
        let model = glm::translate(&glm::Mat4::identity(), &self.model);

        let mvp = proj * view * model;

        self.shader.bind();
        self.shader.uniform_mat4("u_MVP", &mvp);
        self.phone_texture.bind(0);
        self.rust_texture.bind(1);

        gl_call!({
            gl::BindTextureUnit(0, self.phone_texture.renderer_id());
            gl::BindTextureUnit(1, self.rust_texture.renderer_id());

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, std::ptr::null());
        });
    }

    fn imgui_render(&mut self, _: (f32, f32), ui: &imgui_glfw_rs::imgui::Ui) {
        let model = self.model.as_mut_slice();
        ui.slider_float3(
            &str_to_imstr("Model Transform"),
            model.try_into().unwrap(),
            0.0,
            600.,
        )
        .build();
    }

    fn update(&mut self, _delta_time: f32) {}
}

impl TestableID for TestBatchRendering {
    fn test_id() -> String {
        "batch_rendering".into()
    }

    fn test_name() -> String {
        "Batch Rendering".into()
    }
}
