#![feature(alloc_error_handler)]

#![no_main]
#![no_std]
#![windows_subsystem = "windows"]

extern crate alloc;
extern crate winapi;

mod memory;
mod window;
use window::{create_window, handle_message};

struct Game {
    renderables: alloc::vec::Vec<&'static str>,
}

lazy_static::lazy_static! {
    static ref GAME: Game = Game {
        renderables: ::alloc::vec!["hi\0", "\nthere\0"],
    };
}

#[panic_handler]
fn panic( _info: &core::panic::PanicInfo ) -> ! { loop {} }

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    let window = create_window();
    loop {
        if !handle_message(window) {
            break;
        }
    }
}
