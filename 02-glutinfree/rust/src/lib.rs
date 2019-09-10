extern crate glutin;
extern crate raw_window_handle;

mod renderer;

use raw_window_handle::windows::WindowsHandle;
use renderer::Renderer;
use std::mem::transmute;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn create_renderer(hwnd: *mut c_void, width: f64, height: f64) -> *mut Renderer {
    let size = glutin::dpi::PhysicalSize {
        width: width,
        height: height,
    };

    let handle = raw_window_handle::RawWindowHandle::Windows(WindowsHandle {
        hwnd,
        ..WindowsHandle::empty()
    });

    let renderer = Renderer::new(handle, size);

    unsafe { transmute(Box::new(renderer)) }
}

#[no_mangle]
pub extern "C" fn renderer_draw(renderer: *mut Renderer) {
    let r = unsafe { &mut *renderer };
    r.draw();
}

#[no_mangle]
pub extern "C" fn renderer_resize(renderer: *mut Renderer, width: f64, height: f64) {
    let r = unsafe { &mut *renderer };
    let size = glutin::dpi::PhysicalSize {
        width: width,
        height: height,
    };

    r.resize(size);
}

#[no_mangle]
pub extern "C" fn destroy_renderer(renderer: *mut Renderer) {
    let _r: Box<Renderer> = unsafe { transmute(renderer) };
    // Let the box drop
}
