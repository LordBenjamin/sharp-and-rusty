extern crate winit;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::mem::transmute;
use std::ffi::c_void;

pub struct WindowWrapper {
    hwnd: *mut c_void,
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
}

impl WindowWrapper {
    fn new() -> WindowWrapper {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        use winit::platform::windows::WindowExtWindows;
        let hwnd = unsafe { transmute(window.hwnd()) };

        WindowWrapper {
            event_loop: event_loop,
            window: window,
            hwnd: hwnd,
        }
    }

    fn run(&self) {
        // TODO: Make this compile!

        /* self.event_loop.run(move |event, _, control_flow| {
            match event {
                _ => *control_flow = ControlFlow::Wait,
            }
        }); */
    }
}

#[no_mangle]
pub extern "C" fn create_window() -> *mut WindowWrapper {
    unsafe { transmute(Box::new(WindowWrapper::new())) }
}

// TODO: Call from C# to pump messages
#[no_mangle]
pub extern "C" fn run_event_loop(wrapper: *const WindowWrapper ) {
    let window_wrapper = unsafe { (&*wrapper) };
    window_wrapper.run();
}