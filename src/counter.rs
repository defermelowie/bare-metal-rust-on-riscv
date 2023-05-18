// Source: https://docs.rust-embedded.org/book/peripherals/index.html

/// Holds registers for the counter peripheral
///
/// This is a C-style struct since its memory layout should be fixed
#[repr(C)]
struct Counter_Registers {
    cr: u32, // Control register
    sr: u32, // Status register
    vr: u32, // Value register
}

/// Holds the counter peripheral
///
/// Has one member which is a reference to the counter's registers.
/// These registers are:
/// * `status_reg`
/// * `control_reg`
/// * `value_reg`
///
/// Furthermore the const generic `ADDRESS` is used to specify it's base address at compile time
pub struct Counter<const ADDRESS: u32> {
    reg: &'static mut Counter_Registers,
}

impl <const A: u32> Counter<A> {
    /// Create a new counter with a fixed base address
    pub fn new() -> Self{
        Counter {
            reg: unsafe { &mut *(A as *mut Counter_Registers) },
        }
    }

    /// Get `status_reg`'s value
    pub fn get_status_reg(&self) -> u32 {
        unsafe {
            let sr_p: *const u32 = &(self.reg.sr); // Take pointer to status register
            sr_p.read_volatile() // Read from pointer
        }
    }

    /// Get `control_reg`'s value
    pub fn get_control_reg(&self) -> u32 {
        unsafe {
            let cr_p: *const u32 = &(self.reg.cr); // Take pointer to command register
            cr_p.read_volatile() // Read from pointer
        }
    }

    /// Get `value_reg`'s value
    pub fn get_value(&self) -> u32 {
        unsafe {
            let value_p: *const u32 = &(self.reg.vr); // Take pointer to value register
            value_p.read_volatile() // Read from pointer
        }
    }

    /// Set `control_reg`'s value
    pub fn set_control_reg(&mut self, value: u32) -> () {
        unsafe {
            let cr_p: *mut u32 = &mut (self.reg.cr); // Take mutable pointer to command register
            cr_p.write_volatile(value) // Write to pointer
        };
    }
}
