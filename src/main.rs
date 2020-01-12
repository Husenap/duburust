extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;

use std::sync::mpsc::Receiver;

const SRC_WIDTH: u32 = 800;
const SRC_HEIGHT: u32 = 600;

trait Foo {
    fn test(&self) -> String;
}

impl Foo for String {
    fn test(&self) -> String {
        return format!("from String: {}", self);
    }
}
impl Foo for f64 {
    fn test(&self) -> String {
        return format!("from f64: {}", self);
    }
}
impl Foo for f32 {
    fn test(&self) -> String {
        return format!("from f32: {}", self);
    }
}

fn main() {
    let x: f32 = 5.3;
    let y: f64 = 5.3;
    let z: String = "안녕하세요".to_string();
    println!("{}", Foo::test(&x));
    println!("{}", Foo::test(&y));
    println!("{}", Foo::test(&z));

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(SRC_WIDTH, SRC_HEIGHT, "두부입니다", glfw::WindowMode::Windowed)
                                   .expect("Failed to create GLFW window!");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    while !window.should_close() {
        process_events(&mut window, &events);

        let t = glfw.get_time() as f32;

        unsafe {
            let (s, c) = t.sin_cos();
            gl::ClearColor(0.5 + 0.5 * c, 0.5 + 0.5 * s, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe { gl::Viewport(0, 0, width, height) },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => println!("YOU PRESSED A!"),
            _ => {}
        }
    }
}
