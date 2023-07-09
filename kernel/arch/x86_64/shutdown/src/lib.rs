#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ShutdownExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn shutdown(exit_code: ShutdownExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

