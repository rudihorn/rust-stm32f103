#![allow(dead_code)]

use super::common;
use super::common::{Register};
use ::core::ptr::{write_volatile};

// Port Registers
pub struct PortRegisters {
	// 0x00: Control register lower
	crl : Register,

	// 0x04: Control register higher
	crh: Register,

	// 0x08: Input data register
	idr: Register,

	// 0x0c: Output data register
	odr: Register,

	// 0x10: Bit set / reset register
	bsrr: Register,
}

pub enum PortPinMode {
    InputMode = 0,
    OutputMode10MHz = 1,
    OutputMode2MHz = 2,
    OutputMode50MHz = 3,
}

pub enum PortPinConfig {
    AnalogMode = 0,
    FloatingInput = 1,
    InputPullUpDown = 2,
    Reserved = 3,
    GeneralPurposeOutputPushPull = 4,
    GeneralPurposeOutputOpenDrain = 5,
    AlternateFunctionOutputPushPull = 6,
    AlternateFunctionOutputOpenDrain = 7,
}

pub enum Port {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

pub enum Pin { P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15 }


impl PortRegisters {
    pub fn set_port_pin_config(&mut self, pin : Pin, config : PortPinConfig) {
        let pin = pin as u32;
        let config = (config as u32) & 0b0011;
        if pin >= 8 {
            let pin = pin - 8;
            self.crh = common::replace(self.crh, config, 0b0011, pin * 4 + 2);
        } else {
            self.crl = common::replace(self.crl, config, 0x0011, pin * 4 + 2);
        }
    }

    pub fn set_port_pin_mode(&mut self, pin : Pin, mode : PortPinMode) {
        unsafe {
            let pin = pin as u32;
            if pin >= 8 {
                let pin = pin - 8;
                write_volatile(&mut(self.crh), common::replace(self.crh, mode as u32, 0b0011, pin * 4));
            } else {
                write_volatile(&mut(self.crl), common::replace(self.crl, mode as u32, 0x0011, pin * 4));
            } 
        }
    }
    
    pub fn set_port_pin_out(&mut self, pin : Pin) {
        let pin = pin as u32;
        unsafe{write_volatile(&mut(self.bsrr), 1 << pin);}
    }

    pub fn clear_port_pin_out(&mut self, pin : Pin) {
        let pin = pin as u32;
        self.bsrr = 1 << (pin + 16);
    }
}


