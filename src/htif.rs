#[no_mangle]
#[used]
#[link_section = ".tohost"]
static mut tohost: Wrapper = Wrapper([0x0; 0x20]);

#[no_mangle]
#[used]
#[link_section = ".tohost"]
static mut fromhost: Wrapper = Wrapper([0x0; 0x20]);

#[repr(C, align(0x100))]
struct Wrapper([u64; 0x20]);

fn write_tohost(msg: u64) {
    unsafe {
        tohost.0[0] = msg;
        tohost.0[1] = 0x0;
    };
}

/// Exit emulator & signal success
pub fn exit_success() {
    loop {
        write_tohost(0x1)
    }
}

/// Exit emulator & signal failure cause
///
/// _Note that `0` can't be a failure cause_
pub fn exit_failure(sig: u64) {
    if sig == 0 {
        panic!()
    } else {
        loop {
            write_tohost((sig << 1) | 1);
        }
    }
}
