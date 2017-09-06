#![allow(dead_code)]

use super::common;
use super::common::{Register};
use ::core::ptr::{write_volatile, read_volatile};

pub struct USARTRegisters {
    // 0x00 : Status Register
    pub status: Register,

    // 0x04 : Data register
    pub dr: Register,

    // 0x08 : Baud rate register_bank
    pub brr: Register,

    // 0x0C : Control register 1
    pub cr1: Register,

    // 0x10 : Control register 2
    pub cr2: Register,

    // 0x14 : Control register 3
    pub cr3: Register,

    // 0x18 : Guard time and prescaler register_bank
    pub gtpr: Register,
}

impl USARTRegisters {

    pub fn set_baud_calc(&mut self, clk : u32, baud : u32) {
        let div = (clk) / baud;
        return self.set_baud(div / 16, div % 16);
    }

    read_bit_fns!(get_transmit_empty, status, 7);
    read_bit_fns!(get_read_not_empty, status, 5);

    write_bit_fns!(enable, disable, cr1, 13);
    write_bit_fns!(enable_transmitter, disable_transmitter, cr1, 3);
    write_bit_fns!(enable_receiver, disable_receiver, cr1, 2);

    pub fn send_data(&mut self, val : u8) {
        let val = val as u32;
        unsafe { write_volatile(&mut(self.dr), val) };
    }

    pub fn set_baud(&mut self, mantissa:u32, fraction:u32) {
        let brr = (fraction & 0b1111) | (mantissa << 4);
        unsafe { write_volatile(&mut(self.brr), brr) };
    }
}
