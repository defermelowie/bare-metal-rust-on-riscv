// Source: https://docs.rust-embedded.org/book/peripherals/index.html

#[repr(C)]
struct Counter_Registers {
    cr: u32, // Control register
    sr: u32, // Status register
    vr: u32, // Value register
}

pub struct Counter {
    reg: &'static mut Counter_Registers,
}

impl Counter {
    pub fn new(counter_base_address: u32) -> Counter {
        Counter {
            reg: unsafe { &mut *(counter_base_address as *mut Counter_Registers) },
        }
    }

    pub fn get_status_reg(&self) -> u32 {
        unsafe {
            let sr_p: *const u32 = &(self.reg.sr);
            sr_p.read_volatile()
        }
    }

    pub fn get_command_reg(&self) -> u32 {
        unsafe {
            let cr_p: *const u32 = &(self.reg.cr);
            cr_p.read_volatile()
        }
    }

    pub fn get_value(&self) -> u32 {
        unsafe {
            let value_p: *const u32 = &(self.reg.vr);
            value_p.read_volatile()
        }
    }

    pub fn set_command_reg(&mut self, value: u32) -> () {
        unsafe {
            let cr_p: *mut u32 = &mut (self.reg.cr);
            cr_p.write_volatile(value)
        };
    }
}
