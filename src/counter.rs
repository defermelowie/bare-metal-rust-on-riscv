// Source: https://docs.rust-embedded.org/book/peripherals/index.html

#[repr(C)]
struct Counter_Registers {
    cr: u32,
    sr: u32,
    value: u32,
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

    pub fn get_command_reg(&self) -> u32 {
        unsafe {
            let cr_p: *const u32 = &(self.reg.cr);
            cr_p.read_volatile()
        }
    }

    pub fn set_command_reg(&mut self, value: u32) -> () {
        unsafe {
            let cr_p: *mut u32 = &mut (self.reg.cr);
            cr_p.write_volatile(value)
        };
    }

    pub fn get_value(&self) -> u32 {
        unsafe {
            let value_p: *const u32 = &(self.reg.value);
            value_p.read_volatile()
        }
    }

    pub fn set_value(&mut self, value: u32) -> () {
        unsafe {
            let value_p: *mut u32 = &mut (self.reg.value);
            value_p.write_volatile(value)
        };
    }
}
