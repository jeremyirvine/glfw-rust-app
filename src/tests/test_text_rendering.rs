use crate::{
    gl_component::GLComponent, index_buffer::IndexBuffer, shader::Shader,
    vertex_array::VertexArray, vertex_buffer::VertexBuffer,
    vertex_buffer_layout::VertexBufferLayout, ShaderBuilder,
};

use super::{Testable, TestableID};
use glcall_macro::gl_call;
use image::{EncodableLayout, Rgba, RgbaImage};
use memoffset::offset_of;
use nalgebra_glm as glm;
use rusttype::{point, Font, Scale, VMetrics};
use std::{ffi::c_void, collections::HashMap};

#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture_coords: [f32; 2],
    pub texture_index: f32,

    renderer_id: u32,
}

impl Vertex {
    pub fn new(position: [f32; 3], texture_coords: [f32; 2], texture_index: f32) -> Self {
        Self {
            position,
            texture_coords,
            texture_index,

            renderer_id: 0,
        }
    }
}

pub fn gen_quad_vertices(x: f32, y: f32, width: f32, height: f32, texture_index: f32) -> Vec<Vertex> {
    let size = 300.0;

    #[rustfmt::skip]
    let vertices = vec![
        Vertex::new([x,         x,          0.0], [0.0, 1.0], texture_index),
        Vertex::new([x + width, x,          0.0], [1.0, 1.0], texture_index),
        Vertex::new([x + width, x + height, 0.0], [1.0, 0.0], texture_index),
        Vertex::new([x,         x + height, 0.0], [0.0, 0.0], texture_index),
    ];

    vertices
}

struct TextRenderer { 
    font: Font<'static>,
    scale: Scale,
    v_metrics: VMetrics,    

    texture_ids: Vec<u32>,
}

impl Default for TextRenderer {
    fn default() -> Self {
        let font_data = include_bytes!("../res/fonts/Default.ttf").to_vec();
        let font = Font::try_from_vec(font_data).unwrap();

        let height = 12.4;
        let scale = Scale::uniform(42.0);
        let v_metrics = font.v_metrics(scale);

        Self {
            font, scale, v_metrics, texture_ids: vec![],
        }
    }
}

impl TextRenderer {
    pub fn generate_texture(&mut self, s: impl AsRef<str>) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
        let offset = point(0.0, self.v_metrics.ascent);
        let glyphs = self.font.layout(s.as_ref(), self.scale, offset);
        let mut image = RgbaImage::new(1280, 960);

        for g in glyphs {
            if let Some(bounding_box) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    image.put_pixel(
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        Rgba([255, 255, 255, (v * 255.0) as u8]),
                    )
                });
            }
        }

        let mut texture_id = 0;
        gl_call!({
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_bytes().as_ptr() as *const c_void,
            );
        });

        self.texture_ids.push(texture_id);

        image
    }

    pub fn bind_units(&self) {
        for (i, id) in self.texture_ids.iter().enumerate() {
            gl_call!({
                gl::BindTextureUnit(i as u32, *id);
            });
        }
    }

    pub fn clean_all(&mut self) {
        gl_call!({
            gl::DeleteTextures(self.texture_ids.len() as i32, self.texture_ids.as_ptr())
        });

        self.texture_ids = vec![];
    }

    pub fn ids(&self) -> Vec<u32> {
        self.texture_ids.clone()
    }
}

impl Drop for TextRenderer {
    fn drop(&mut self) {
        self.clean_all();
    }
}

pub struct TestTextRendering {
    vbo: VertexBuffer,
    ibo: IndexBuffer,
    vao: VertexArray,

    shader: Shader,

    text_renderer: TextRenderer,
    
    test_text: Option<image::ImageBuffer<Rgba<u8>, Vec<u8>>>,
    test_number: usize,
}

impl Default for TestTextRendering {
    fn default() -> Self {
        let mut text_renderer = TextRenderer::default();

        let vertices = gen_quad_vertices(0., 0., 1280., 960., 0.);
        let indices = vec![0, 1, 2, 2, 3, 0];

        let vao = VertexArray::new();

        let vbo = VertexBuffer::new(&vertices);
        gl_call!({
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, position) as *const c_void,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, texture_coords) as *const c_void,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                1,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                offset_of!(Vertex, texture_index) as *const c_void,
            );
        });

        let ibo = IndexBuffer::new(&indices);

        let shader = ShaderBuilder::default()
            .with_shader("src/res/shaders/Text.glsl")
            .unwrap()
            .build();

        shader.bind();
        shader.uniform_1i("u_Texture", 0);
        shader.unbind();

        TestTextRendering {
            vao,
            ibo,
            vbo,
            shader,
            text_renderer,
            test_text: None,
            test_number: 0,
        }
    }
}

impl Testable for TestTextRendering {
    fn render(&self, screen_size: (f32, f32), _renderer: &crate::renderer::Renderer) {
        let (width, height) = screen_size;
        self.text_renderer.bind_units();
        self.vao.bind();
        self.shader.bind();

        let proj = glm::ortho(0.0, width, 0.0, height, -1.0, 1.0);
        let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, 0.0));
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, 0.0));

        let mvp = proj * view * model;

        self.shader.uniform_mat4("u_MVP", &mvp);
        self.shader.uniform_1iv("u_Textures", vec![0,1]);

        gl_call!({
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, std::ptr::null());
        });
    }
    fn imgui_render(&mut self, _screen_size: (f32, f32), _ui: &imgui_glfw_rs::imgui::Ui) {}
    fn update(&mut self, _delta_time: f32) {
        self.text_renderer.clean_all();

        let image = self.text_renderer.generate_texture(format!("Number: {}", self.test_number));
        let image2 = self.text_renderer.generate_texture(format!("Number 2: {}", self.test_number * 2));
        self.test_text = Some(image);

        self.test_number += 1;
    }
}

impl TestableID for TestTextRendering {
    fn test_id() -> String {
        "text_rendering".into()
    }

    fn test_name() -> String {
        "Text Rendering".into()
    }
}
