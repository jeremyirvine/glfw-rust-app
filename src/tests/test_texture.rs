use crate::{
    gl_component::GLComponent, index_buffer::IndexBuffer, renderer::Renderer, shader::Shader,
    str_to_imstr, texture::Texture, vertex_array::VertexArray, vertex_buffer::VertexBuffer,
    vertex_buffer_layout::VertexBufferLayout, ShaderBuilder,
};
use gl::types::GLuint;
use glcall_macro::gl_call;
use nalgebra_glm as glm;

use super::{Testable, TestableID};

pub struct TestTexture {
    vao: VertexArray,
    ibo: IndexBuffer,
    shader: Shader,
    texture: Texture,
    view: glm::Mat4,
    model: glm::Vec3,
}

impl Default for TestTexture {
    fn default() -> Self {
        #[rustfmt::skip]
        let vertices: Vec<f32> = vec![
            -50.0, -50.0, 0.0,   0.0, 0.0,
             50.0, -50.0, 0.0,   1.0, 0.0,
             50.0,  50.0, 0.0,   1.0, 1.0,
            -50.0,  50.0, 0.0,   0.0, 1.0,
        ];

        #[rustfmt::skip]
        let indices: Vec<GLuint> = vec![
            0,1,2,
            2,3,0
        ];

        let shader = ShaderBuilder::default()
            .with_shader_source(include_str!("../res/shaders/Default.glsl").into())
            .expect("Failed to build shader from source")
            .build();

        let layout = VertexBufferLayout::default().with_floats(3).with_floats(2);

        let mut vao = VertexArray::new();

        let vbo = VertexBuffer::new(&vertices);
        let ibo = IndexBuffer::new(&indices);
        vao.add_buffer(&vbo, &layout);

        vao.unbind();
        vbo.unbind();
        ibo.unbind();

        let texture = Texture::new("src/res/textures/phone.png", 0);
        texture.bind(0);
        shader.bind();
        shader.uniform_1i("u_Texture", 0);
        shader.unbind();

        let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(0., 0., 0.));
        let model = glm::vec3(100., 100., 0.);

        Self {
            vao,
            ibo,
            shader,
            view,
            model,
            texture,
        }
    }
}

impl Testable for TestTexture {
    fn render(&self, screen_size: (f32, f32), renderer: &Renderer) {
        let (width, height) = screen_size;
        let proj = glm::ortho(0.0, width, 0.0, height, -1.0, 1.0);
        let model = glm::translate(&glm::Mat4::identity(), &self.model);
        let mvp = proj * self.view * model;

        gl_call!({
            gl::ClearColor(0.2, 0.3, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        });

        self.shader.bind();
        self.texture.bind(0);
        self.shader.uniform_mat4("u_MVP", &mvp);

        self.vao.bind();

        renderer.draw(&self.vao, &self.ibo, &self.shader);
        self.shader.unbind();
    }

    fn imgui_render(&mut self, (width, height): (f32, f32), ui: &imgui_glfw_rs::imgui::Ui) {
        ui.slider_float3(
            &str_to_imstr("Model Translation"),
            self.model.as_mut(),
            0.0,
            width.max(height),
        )
        .build();
    }

    fn update(&mut self, _delta_time: f32) {}
}

impl TestableID for TestTexture {
    fn test_id() -> String {
        "test_texture".into()
    }

    fn test_name() -> String {
        "Texture".into()
    }
}
