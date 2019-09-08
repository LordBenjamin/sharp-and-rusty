extern crate glutin;
extern crate gl;

mod renderer;

use renderer::Renderer;
use std::os::raw::c_void;

#[no_mangle]
pub extern fn create_renderer(hwnd: *mut c_void) -> Box<Renderer> {
    let renderer = Renderer::new(hwnd);

    Box::new(renderer)
}

#[no_mangle]
pub extern fn renderer_draw(mut renderer: Box<Renderer>) {
    renderer.draw();
}