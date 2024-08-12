use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt;

// initial a static mutable variable

lazy_static!{
    pub static ref SERIAL1: Mutex<SerialPort> = Mutex::new({
        let mut serial_port = unsafe{SerialPort::new(0x3F8)};
        serial_port.init();
        serial_port
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(||{
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
    });
}