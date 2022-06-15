#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

mod counter;
use counter::Counter;

/// A panic handler is required in Rust, this is probably the most basic one possible
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Initializes registers and calls `main()`
#[no_mangle]
extern "C" fn init() -> ! {
    unsafe {
        // Zero-initialize all registers
        asm!(
            "addi x1, zero, 0",
            "addi x2, zero, 0",
            "addi x3, zero, 0",
            "addi x4, zero, 0",
            "addi x5, zero, 0",
            "addi x6, zero, 0",
            "addi x7, zero, 0",
            "addi x8, zero, 0",
            "addi x9, zero, 0",
            "addi x10, zero, 0",
            "addi x11, zero, 0",
            "addi x12, zero, 0",
            "addi x13, zero, 0",
            "addi x14, zero, 0",
            "addi x15, zero, 0",
            "addi x16, zero, 0",
            "addi x17, zero, 0",
            "addi x18, zero, 0",
            "addi x19, zero, 0",
            "addi x20, zero, 0",
            "addi x21, zero, 0",
            "addi x22, zero, 0",
            "addi x23, zero, 0",
            "addi x24, zero, 0",
            "addi x25, zero, 0",
            "addi x26, zero, 0",
            "addi x27, zero, 0",
            "addi x28, zero, 0",
            "addi x29, zero, 0",
            "addi x30, zero, 0",
            "addi x31, zero, 0"
        );
        // Set stack pointer
        asm!("lui sp, %hi(16*1024)", "addi sp, sp, %lo(16*1024)",)
    };
    // Call main function
    main();
    // Wait forever (Never return)
    loop {}
}

/// Main program function
fn main() -> () {
    let mut counter = Counter::new(0x8000_0000);
    counter.set_command_reg(0xaaaa);
    let c_s = counter.get_status_reg();
    let c_c = counter.get_command_reg();
    let c_v = counter.get_value();
}
