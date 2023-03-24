#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

extern crate nalgebra_glm as glm;

use std::ffi::CStr;
use std::sync::mpsc::Receiver;

use gl::types::GLuint;
use glcall_macro::gl_call;
use glfw::{Action, Context, Key, OpenGlProfileHint, SwapInterval, WindowHint, WindowMode};
use glfw_app::ShaderBuilder;

use glfw_app::gl_component::GLComponent;
use glfw_app::index_buffer::IndexBuffer;
use glfw_app::renderer::Renderer;
use glfw_app::texture::Texture;
use glfw_app::vertex_array::VertexArray;
use glfw_app::vertex_buffer::VertexBuffer;
use glfw_app::vertex_buffer_layout::VertexBufferLayout;

use glm::Vec3;
use glm::Vec4;
use imgui_glfw_rs::ImguiGLFW;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::glfw;
use imgui::Context as ImContext;

use glfw_app::{gl_log_errors, gl_clear_errors};
use imgui_glfw_rs::imgui::EditableColor;
use imgui_glfw_rs::imgui::ImString;
use imgui_glfw_rs::imgui::sys::ImVec4;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 960;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "LearnOpenGL",
            WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_all_polling(true);
    window.set_framebuffer_size_polling(true);

    glfw.set_swap_interval(SwapInterval::Sync(1));

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut imgui = ImContext::create();

    let mut imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);

    #[rustfmt::skip]
    let vertices: Vec<f32> = vec![
        100.0,  100.0, 0.0,   0.0, 0.0,
        200.0,  100.0, 0.0,   1.0, 0.0,
        200.0,  200.0, 0.0,   1.0, 1.0,
        100.0,  200.0, 0.0,   0.0, 1.0,
    ];

    let indices: Vec<GLuint> = vec![0, 1, 2, 2, 3, 0];

    gl_call!({
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    });

    let shader = ShaderBuilder::default()
        .with_shader_source(include_str!("res/shaders/Default.glsl").into())
        .expect("Failed to build shader from source")
        .build();

    let layout = VertexBufferLayout::default().with_floats(3).with_floats(2);
    let mut vao = VertexArray::new();

    let vbo = VertexBuffer::new(&vertices);
    let ibo = IndexBuffer::new(&indices);

    vao.add_buffer(&vbo, &layout);

    vbo.unbind();
    ibo.unbind();
    vao.unbind();

    shader.bind();
    shader.uniform_4f("u_Color", (1.0, 0.5, 0.0, 1.0));

    let renderer = Renderer::new((0.3, 0.4, 0.8, 1.0));

    let texture = Texture::new("src/res/textures/phone.png".into());
    texture.bind(0);
    shader.uniform_1i("u_Texture", 0);

    {
        let proj = glm::ortho(
            0.0,
            SCREEN_WIDTH as f32,
            0.0,
            SCREEN_HEIGHT as f32,
            -1.0,
            1.0,
        );
        let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(-100., 0., 0.));
        let model = glm::translate(&glm::Mat4::identity(), &glm::vec3(200., 200., 0.));
        let mvp = proj * view * model;
        shader.uniform_mat4("u_MVP", &mvp);
    }

    let mut clear_color = Vec4::new(0.45, 0.55, 0.60, 1.0);

    let mut r = 0.0;
    let mut inc = 0.05;
    while !window.should_close() {
        process_events(&mut window, &events);
        let ui = imgui_glfw.frame(&mut window, &mut imgui);
        ui.window(&ImString::from("Debug".to_string()))
            .size([500.0, 10.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.color_edit(
                    &ImString::from("Clear Color".to_string()), 
                    EditableColor::Float4(clear_color.as_mut_slice().try_into().unwrap()),
                )
                .build();
            });

        renderer.clear();



        shader.bind();
        shader.uniform_4f("u_Color", (r, 0.3, 0.3, 1.0));
        vao.bind();
        ibo.bind();
        renderer.draw(&vao, &ibo, &shader);
        shader.unbind();

        imgui_glfw.draw(ui, &mut window);

        if r < 0.0 || r > 1.0 {
            inc *= -1.;
        }
        r += inc;

        window.swap_buffers();
        glfw.poll_events();
        for (_,event) in glfw::flush_messages(&events) {
            imgui_glfw.handle_event(&mut imgui, &event);
        }
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
