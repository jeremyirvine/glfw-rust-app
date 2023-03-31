use std::ffi::c_void;

use gl::types::GLuint;
use glcall_macro::gl_call;
use memoffset::offset_of;
use nalgebra_glm as glm;

use crate::{
    gl_component::GLComponent, shader::Shader, str_to_imstr, tests::Testable, texture::Texture,
    ShaderBuilder,
};

use super::TestableID;

pub fn gen_quad_indices(count: u32) -> Vec<u32> {
    let i = (0..count).map(|c| {
        vec![0, 1, 2, 2, 3, 0]
            .iter()
            .map(|i| i + (c * 4))
            .collect::<Vec<u32>>()
    });
    i.into_iter().flatten().collect()
}

pub fn gen_quad_vertices(x: f32, y: f32, texture_index: f32) -> Vec<Vertex> { 
    let size = 100.0;

    #[rustfmt::skip]
    let vertices = vec![
        Vertex::new([x,        y,        0.0],  [0.18, 0.6, 0.96, 1.0], [0.0, 0.0], texture_index),
        Vertex::new([x + size, y,        0.0],  [0.18, 0.6, 0.96, 1.0], [1.0, 0.0], texture_index),
        Vertex::new([x + size, y + size, 0.0],  [0.18, 0.6, 0.96, 1.0], [1.0, 1.0], texture_index),
        Vertex::new([x,        y + size, 0.0],  [0.18, 0.6, 0.96, 1.0], [0.0, 1.0], texture_index),
    ];

    vertices
}

#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub texture_coords: [f32; 2],
    pub texture_index: f32,

    renderer_id: u32,
}

impl Vertex {
    pub fn new(
        position: [f32; 3],
        color: [f32; 4],
        texture_coords: [f32; 2],
        texture_index: f32,
    ) -> Self {
        Self {
            position,
            color,
            texture_coords,
            texture_index,

            renderer_id: 0,
        }
    }
}

pub struct TestBatchRendering {
    vao: u32,
    vbo: u32,
    shader: Shader,

    model: glm::Vec3,

    phone_texture: Texture,
    rust_texture: Texture,

    quad_0_position: [f32; 2],
}

impl Default for TestBatchRendering {
    fn default() -> Self {
        let indices = gen_quad_indices(2);

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
                std::mem::size_of::<Vertex>() as isize * 1000,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            // Vertices
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, position) as *const c_void,
            );

            // Color
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, color) as *const c_void,
            );

            // Texture Coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, texture_coords) as *const c_void,
            );

            // Texture Index
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                1,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, texture_index) as *const c_void,
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
        shader.uniform_1iv("u_Textures", vec![0, 1]);
        shader.unbind();

        Self {
            shader,
            vao,
            vbo,
            model: glm::vec3(200., 200., 0.),
            phone_texture,
            rust_texture,
            quad_0_position: [100., 100.],
        }
    }
}

impl Testable for TestBatchRendering {
    fn render(&self, (width, height): (f32, f32), _renderer: &crate::renderer::Renderer) {
        let x = self.quad_0_position[0];
        let y = self.quad_0_position[1];

        let mut vertices = Vec::<Vertex>::new();
        vertices.extend(gen_quad_vertices(x, y, 0.0));
        vertices.extend(gen_quad_vertices(0., 0., 1.0));

        gl_call!({
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                &vertices[0] as *const Vertex as *const c_void,
            );

            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        });
        let proj = glm::ortho(0.0, width, 0.0, height, -1.0, 1.0);
        let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, 0.0));
        let model = glm::translate(&glm::Mat4::identity(), &self.model);

        let mvp = proj * view * model;

        self.shader.bind();
        self.shader.uniform_mat4("u_MVP", &mvp);

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

        ui.drag_float2(&str_to_imstr("Quad 1 Position"), &mut self.quad_0_position).build();
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
