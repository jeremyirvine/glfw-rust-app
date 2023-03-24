#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

extern crate nalgebra_glm as glm;

use std::sync::mpsc::Receiver;

use gl::types::GLuint;
use glcall_macro::gl_call;
use glfw::{Action, Context, Key, OpenGlProfileHint, SwapInterval, WindowHint, WindowMode};
use glfw_app::ShaderBuilder;

use glfw_app::gl_component::GLComponent;
use glfw_app::index_buffer::IndexBuffer;
use glfw_app::renderer::Renderer;
use glfw_app::str_to_imstr;
use glfw_app::texture::Texture;
use glfw_app::vertex_array::VertexArray;
use glfw_app::vertex_buffer::VertexBuffer;
use glfw_app::vertex_buffer_layout::VertexBufferLayout;

use glm::Vec4;
use imgui::Context as ImContext;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

use glfw_app::{gl_clear_errors, gl_log_errors};
use imgui_glfw_rs::imgui::EditableColor;

fn main() {
    let mut screen_width: u32 = 1280;
    let mut screen_height: u32 = 960;

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            screen_width,
            screen_height,
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

    #[rustfmt::skip]
    let indices: Vec<GLuint> = vec![
        0, 1, 2,
        2, 3, 0
    ];

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

    let mut clear_color = Vec4::new(0.45, 0.55, 0.60, 1.0);
    let mut translate = glm::vec3(200., 200., 0.);

    let mut renderer = Renderer::from(clear_color);

    let texture = Texture::new("src/res/textures/phone.png".into());
    texture.bind(0);
    shader.bind();
    shader.uniform_1i("u_Texture", 0);
    shader.unbind();

    let view = glm::translate(&glm::Mat4::identity(), &glm::vec3(-100., 0., 0.));

    while !window.should_close() {
        process_events(&mut window, &events, &mut screen_width, &mut screen_height);
        let ui = imgui_glfw.frame(&mut window, &mut imgui);
        ui.window(&str_to_imstr("Debug Options"))
            .size([500.0, 10.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.color_edit(
                    &str_to_imstr("Clear Color"),
                    EditableColor::Float4(clear_color.as_mut_slice().try_into().unwrap()),
                )
                .build();

                ui.slider_float3(&str_to_imstr("Model Translation"), translate.as_mut_slice().try_into().unwrap(), 0.0, 960.0).build();
            });
        renderer.set_clear_color_from_vec4(&clear_color);
        renderer.clear();

        let proj = glm::ortho(
            0.0,
            screen_width as f32,
            0.0,
            screen_height as f32,
            -1.0,
            1.0,
        );
        let model = glm::translate(&glm::Mat4::identity(), &translate);
        let mvp = proj * view * model;

        shader.bind();
        shader.uniform_mat4("u_MVP", &mvp);
        vao.bind();
        ibo.bind();
        renderer.draw(&vao, &ibo, &shader);
        shader.unbind();

        imgui_glfw.draw(ui, &mut window);

        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            imgui_glfw.handle_event(&mut imgui, &event);
        }
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>,
    screen_width: &mut u32,
    screen_height: &mut u32,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                *screen_width = width as u32;
                *screen_height = height as u32;
                gl::Viewport(0, 0, width, height);
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
