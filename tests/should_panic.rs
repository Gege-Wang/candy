#![no_std]
#![no_main]

use candy::exit_qemu;
use candy::println;
use candy::serial_print;
use candy::QemuExitCode;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("[test did not panic]\t");
    should_fail();
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_print!("[ok]\n");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    println!("should_panic:::should_fail");
    assert_eq!(0, 1);
}
