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
pub struct Counter {
    reg: &'static mut Counter_Registers,
}

impl Counter {
    /// Create a new counter with given base address
    pub fn new(counter_base_address: u32) -> Counter {
        Counter {
            reg: unsafe { &mut *(counter_base_address as *mut Counter_Registers) },
        }
    }

    /// Get `status_reg`'s value
    pub fn get_status_reg(&self) -> u32 {
        unsafe {
            let sr_p: *const u32 = &(self.reg.sr); // Take pointer to status register
            sr_p.read_volatile() // Read from pointer
        }
    }

    /// Get `command_reg`'s value
    pub fn get_command_reg(&self) -> u32 {
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

    /// Set `command_reg`'s value
    pub fn set_command_reg(&mut self, value: u32) -> () {
        unsafe {
            let cr_p: *mut u32 = &mut (self.reg.cr); // Take mutable pointer to command register
            cr_p.write_volatile(value) // Write to pointer
        };
    }
}
