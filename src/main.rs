#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

mod counter;
use counter::Counter;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn init() -> ! {
    unsafe {
        asm!(
            /* zero-initialize all registers */
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
            "addi x31, zero, 0",
            /* set stack pointer */
            "lui sp, %hi(16*1024)",
            "addi sp, sp, %lo(16*1024)",
            /* call main */
            "jal ra, main"
        )
    };
    loop {}
}

#[no_mangle]
extern "C" fn main() -> () {
    let mut counter = Counter::new(0x8000_0000);
    counter.set_command_reg(0xaaaa);
    counter.set_value(0x78478);
    let c_c = counter.get_command_reg();
    let c_v = counter.get_value();
}
