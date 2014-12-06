use glfw::Context;
use gfx;
use glfw;

use glfw::{WindowHint, WindowMode};

use errors::{GameError, Res};


pub struct Engine {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Engine {

    pub fn new() -> Res<Engine> {
        let glfw = try!(glfw::init(glfw::FAIL_ON_ERRORS));

        glfw.window_hint(WindowHint::ContextVersion(3, 2));
        glfw.window_hint(WindowHint::OpenglForwardCompat(true));
        glfw.window_hint(WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::Samples(4));
        glfw.window_hint(WindowHint::SRgbCapable(true));
        glfw.window_hint(WindowHint::Resizable(false));

        let (window, events) = unwrap_or!(glfw
            .create_window(1280, 720, "Little Tanks", WindowMode::Windowed),
            return Err(GameError::WindowInitError));

        window.make_current();
        glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
        window.set_key_polling(true);

        Ok(Engine {
            glfw: glfw,
            window: window,
            events: events,
        })
    }

    pub fn get_framebuffer_size(&self) -> (u16, u16) {
        let (w, h) = self.window.get_framebuffer_size();
        (w as u16, h as u16)
    }

    pub fn get_framebuffer_aspect(&self) -> f32 {
        let (w, h) = self.window.get_framebuffer_size();
        w as f32 / h as f32
    }

    pub fn new_frame(&self) -> gfx::Frame {
        let (w, h) = self.get_framebuffer_size();
        gfx::Frame::new(w, h)
    }

    pub fn new_device(&self) -> gfx::GlDevice {
        gfx::GlDevice::new(|s| self.window.get_proc_address(s))
    }
}
