#![no_std]
#![no_main]

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = vga_buffer::Writer {
        column_position: 0,
        color_code: vga_buffer::ColorCode::new(vga_buffer::Color::Yellow, vga_buffer::Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut _) },
    };

    writer
        .write_str("Hello Sh-OS!\nEverything is working!")
        .unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
