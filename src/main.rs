#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

mod counter;
use counter::Counter;

global_asm!(include_str!("init.s"));

/// A panic handler is required in Rust, this is probably the most basic one possible
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Main program function
#[no_mangle]
extern "C" fn main() -> () {
    // Example: Create a counter peripheral with base address 0x8000_0000
    let mut counter = Counter::new(0x8000_0000);
    counter.set_command_reg(0xaaaa);
    let c_s = counter.get_status_reg();
    let c_c = counter.get_command_reg();
    let c_v = counter.get_value();
}
