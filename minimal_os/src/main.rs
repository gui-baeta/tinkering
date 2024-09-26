#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_handler)]
#![reexport_test_harness_main = "test_main"]
#[warn(unused_imports)]

mod test;
mod vga;
mod qemu;

#[cfg(test)]
pub fn test_handler(tests: &[&dyn Fn()]) {
    WRITER.lock().clear_buffer();
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

use crate::vga::vga_buffer;
use core::panic::PanicInfo;
use crate::qemu::{exit_qemu, QemuExitCode};
use crate::vga_buffer::WRITER;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello again");
    println!(", some numbers: {} {}", 42, 1.337);

    #[cfg(test)]
    test_main();

    exit_qemu(QemuExitCode::Success);

    loop {}
}
