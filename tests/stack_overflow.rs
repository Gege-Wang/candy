#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use core::panic::PanicInfo;
use candy::serial_println;
use candy::exit_qemu;
use candy::QemuExitCode;
use candy::gdt::DOUBLE_FAULT_IST_INDEX;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("[stack_overflow double exception]\t");
    init_test_idt();
    candy::gdt::init();
    stack_overflow();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    candy::test_panic_handler(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe{
            idt.double_fault.set_handler_fn(test_double_fault_handler).set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}
extern "x86-interrupt" fn test_double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    serial_println!("ok");
    exit_qemu(QemuExitCode::Success);
    loop {}
    
}
pub fn init_test_idt() {
    TEST_IDT.load();
}

