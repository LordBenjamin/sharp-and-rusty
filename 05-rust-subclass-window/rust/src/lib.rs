extern crate winapi;

use winapi::shared::basetsd::{DWORD_PTR, UINT_PTR};
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::commctrl::{DefSubclassProc, SetWindowSubclass};
use winapi::um::winuser::WM_DESTROY;

#[no_mangle]
pub unsafe extern "C" fn subclass(hwnd: HWND) {
    SetWindowSubclass(hwnd, Some(wnd_proc), 1, 0);
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
    _u_id_subclass: UINT_PTR,
    _dw_ref_data: DWORD_PTR,
) -> LRESULT {
    match message {
        WM_DESTROY => println!("window destroyed!"),
        _ => (),
    };

    DefSubclassProc(hwnd, message, wparam, lparam)
}
