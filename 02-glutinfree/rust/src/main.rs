mod renderer;

extern crate glutin;
extern crate raw_window_handle;
extern crate takeable_option;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use raw_window_handle::windows::WindowsHandle;
use takeable_option::Takeable;

fn main() {
    let events_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Glutin Triangle")
        .build(&events_loop)
        .unwrap();

    use glutin::platform::windows::WindowExtWindows;

    let hwnd = window.hwnd();

    let handle = raw_window_handle::RawWindowHandle::Windows(WindowsHandle {
        hwnd,
        ..WindowsHandle::empty()
    });
    
    let size = window.inner_size().to_physical(window.hidpi_factor()).into();

    let r = renderer::Renderer::new(handle, size);

    // Main loop

    let mut raw_context = Takeable::new(r);

    events_loop.run(move |event, _, control_flow| {
        // println!("el {:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {
                Takeable::take(&mut raw_context); // Make sure it drops first
                return;
            }
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = window.hidpi_factor();
                    let size = logical_size.to_physical(dpi_factor).into();
                    raw_context.resize(size);
                }

                WindowEvent::RedrawRequested => {
                    raw_context.draw();
                }

                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
