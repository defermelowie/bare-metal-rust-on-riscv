#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

mod counter;
use counter::Counter;

mod htif;
use htif::{exit_failure, exit_success};

global_asm!(include_str!("init.s"));

/// A panic handler is required in Rust
///
/// This one exits the emulator with failure cause `1`
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit_failure(1);
    loop {}
}

/// Main program function
#[no_mangle]
extern "C" fn main() -> () {
    // Example: Create a counter peripheral with base address 0x8000_0000
    let mut counter: Counter<0x8000_7000> = Counter::new();
    counter.set_control_reg(0xaaaa);
    let c_s = counter.get_status_reg();
    let c_c = counter.get_control_reg();
    let c_v = counter.get_value();
    exit_success();
}
