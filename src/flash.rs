#![allow(dead_code)]

use super::common;
use super::common::{Register};
use ::core::ptr::{read_volatile, write_volatile};


pub struct FlashRegisters {
    // 0x00 : Access Control Register
    pub acr: Register,

    // 0x04 : 
    pub keyr: Register,

    // 0x08 :
    pub optkeyr: Register,
}

pub enum FlashLatency {
    // 0 Hz < SYSCLK <= 24 MHz
    ZeroWait,
    // 24 MHz < SYSCLK <= 48 MHz
    OneWait,
    // 48 MHz < SYSCLK <= 72 MHz
    TwoWait,
}

impl FlashRegisters {
    pub fn set_latency(&mut self, lat : FlashLatency) {
        let lat = lat as u32;
        unsafe { write_volatile(&mut(self.acr), common::replace(self.acr, lat, 0b111, 0)) };
    }

    read_bit_fns!(get_prefetch_status, acr, 5);

    write_bit_fns!(enable_prefetch, disable_prefetch, acr, 4);
    write_bit_fns!(enable_half_cycle_access, disable_half_cycle_access, acr, 4);
}
