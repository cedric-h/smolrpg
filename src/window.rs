use core::mem::MaybeUninit;

use winapi::shared::minwindef::{LPARAM, LPVOID, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HICON, HMENU, HWND};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::{
    BeginPaint, CreateWindowExA, DefWindowProcA, DispatchMessageA, DrawTextA, EndPaint,
    GetClientRect, GetMessageA, PostQuitMessage, RegisterClassA, TranslateMessage,
};
use winapi::um::winuser::{
    CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, DT_CENTER,
    WNDCLASSA, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

pub fn create_window() -> HWND {
    unsafe {
        let hinstance = GetModuleHandleA(0 as *const i8);
        let wnd_class = WNDCLASSA {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: hinstance,
            lpszClassName: "MyClass\0".as_ptr() as *const i8,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: 0 as HICON,
            hCursor: 0 as HICON,
            hbrBackground: 0 as HBRUSH,
            lpszMenuName: 0 as *const i8,
        };
        RegisterClassA(&wnd_class);

        CreateWindowExA(
            0,                                 // dwExStyle
            "MyClass\0".as_ptr() as *const i8, // class we registered.
            "MiniWIN\0".as_ptr() as *const i8, // title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,  // dwStyle
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT, // size and position
            0 as HWND,     // hWndParent
            0 as HMENU,    // hMenu
            hinstance,     // hInstance
            0 as LPVOID,   // lpParam
        ) 
    }
}

// More info: https://msdn.microsoft.com/en-us/library/windows/desktop/ms644927(v=vs.85).aspx
pub fn handle_message(window: HWND) -> bool {
    unsafe {
        let mut msg = MaybeUninit::uninit();
        if GetMessageA(msg.as_mut_ptr(), window, 0, 0) > 0 {
            TranslateMessage(msg.as_ptr());
            DispatchMessageA(msg.as_ptr());
            true
        } else {
            false
        }
    }
}

pub unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        winapi::um::winuser::WM_PAINT => {
            let mut paint_struct = MaybeUninit::uninit();
            let mut rect = MaybeUninit::uninit();
            let hdc = BeginPaint(hwnd, paint_struct.as_mut_ptr());
            GetClientRect(hwnd, rect.as_mut_ptr());
            for word in super::GAME.renderables.iter() {
                DrawTextA(
                    hdc,
                    word.as_ptr() as *const i8,
                    -1,
                    rect.as_mut_ptr(),
                    DT_CENTER,
                );
            }
            EndPaint(hwnd, paint_struct.as_mut_ptr());
        }
        winapi::um::winuser::WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => {
            return DefWindowProcA(hwnd, msg, wparam, lparam);
        }
    }
    return 0;
}

