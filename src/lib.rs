#![no_std]

// testing
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// exceptions and interrupts
#![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;


pub fn init() {

    // initialize Global Descriptor Table
    // (sets Code Segment and Task State Segment)
    gdt::init();

    // initialize Interrupt Descriptor Table
    interrupts::init_idt();

    // program Programmable Interval Timer
    // see https://wiki.osdev.org/Programmable_Interval_Timer
    unsafe {
        let mut port = Port::new(0x40);
        // write twice because I'm not sure if lo, hi, or both
        port.write(0 as u8);
        port.write(1 as u8);
    }

    // initialize Programmable Interrupt Controllers
    unsafe { interrupts::PICS.lock().initialize() };

    // enable interrupts
    x86_64::instructions::interrupts::enable();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    let mut port = Port::new(0xf4);
    unsafe { port.write(exit_code as u32); }
}
