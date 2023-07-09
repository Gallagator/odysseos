#![no_std]

// Mostly taken from https://os.phil-opp.com/testing/#printing-to-the-console

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn serial_println(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}
