extern crate winapi;

use std::mem;
use std::mem::transmute;
use std::ptr;

use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowLongPtrW, PostQuitMessage,
    SetWindowLongPtrW, TranslateMessage, GWLP_WNDPROC, MSG, WM_DESTROY, WNDPROC,
};

#[no_mangle]
pub unsafe extern "C" fn run_event_loop(hwnd: HWND) {
    let _old_proc: WNDPROC = transmute(GetWindowLongPtrW(hwnd, GWLP_WNDPROC));

    SetWindowLongPtrW(hwnd, GWLP_WNDPROC, wnd_proc as LONG_PTR);

    let mut msg: MSG = mem::zeroed();

    while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let mut result: LRESULT = mem::zeroed();

    match message {
        WM_DESTROY => {
            println!("window destroyed!");
            PostQuitMessage(0)
        }
        _ => result = DefWindowProcW(hwnd, message, wparam, lparam),
    };

    result
}
