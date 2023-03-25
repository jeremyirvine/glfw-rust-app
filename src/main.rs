#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

extern crate nalgebra_glm as glm;

use std::sync::mpsc::Receiver;

use glcall_macro::gl_call;
use glfw::{Action, Context, Key, OpenGlProfileHint, SwapInterval, WindowHint, WindowMode};

use glfw_app::renderer::Renderer;
use glfw_app::str_to_imstr;

use glfw_app::tests::Testable;
use glfw_app::tests::menu::TestMenu;
use glfw_app::tests::test_clear_color::TestClearColor;
use imgui::Context as ImContext;
use imgui_glfw_rs::glfw;
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

use glfw_app::{gl_clear_errors, gl_log_errors};

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

    gl_call!({
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    });

    let renderer = Renderer::new((0.0, 0.0, 0.0, 1.0));
    let mut test_menu = TestMenu::default();
    test_menu.register_test(Box::new(TestClearColor::default()));

    while !window.should_close() {
        renderer.clear();
        process_events(&mut window, &events, &mut screen_width, &mut screen_height);

        let ui = imgui_glfw.frame(&mut window, &mut imgui);
        ui.window(&str_to_imstr(&test_menu.imgui_title()))
            .size([500.0, 100.0], imgui::Condition::FirstUseEver)
            .build(|| {
                test_menu.imgui_render(&ui);
            });

        test_menu.update(0.0);
        test_menu.render();
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
