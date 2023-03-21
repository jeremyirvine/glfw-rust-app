mod shader;

use std::{sync::mpsc::Receiver, os::raw::c_void, ptr};

use glcall_macro::gl_call;
use glfw::{Context, Key, Action, WindowHint, OpenGlProfileHint, WindowMode};
use glfw_app::ShaderBuilder;

const SCREEN_HEIGHT: u32 = 800;
const SCREEN_WIDTH: u32 = 800;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    #[cfg(target_os = "macos")]
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "LearnOpenGL", WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
         0.5, -0.5, 0.0,
         0.0,  0.5, 0.0,
    ];

    let (mut vao, mut vbo) = (0, 0);

    let shader = ShaderBuilder::default()
        .with_shader_source(include_str!("res/Default.glsl").into())
        .expect("Failed to build shader from source")
        .build();

    gl_call!({
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            (vertices.len() * std::mem::size_of::<f64>()) as isize,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            (3 * std::mem::size_of::<f32>()) as i32, 
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    });

    shader.bind(); 
    shader.uniform_4f("u_Color", (1.0, 0.5, 0.0, 1.0));
    while !window.should_close() {
        process_events(&mut window, &events);

        gl_call!({
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        });

        shader.bind();
        gl_call!({
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        });
        shader.unbind();

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width,height) => unsafe { gl::Viewport(0, 0, width, height); },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Release, _) => window.set_should_close(true),
            _ => {},
        }
    }
}
