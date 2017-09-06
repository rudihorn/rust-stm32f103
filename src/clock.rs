#![allow(dead_code)]

use super::common;
use super::common::{Register};
use ::core::ptr::{read_volatile, write_volatile};



// Reset and Clock Control
pub struct RCCRegisters {
	// 0x00: Clock control register
	pub cr: Register, 

	// 0x04: Clock configuration register
	pub cfgr: Register,

	// 0x08: Clock interrupt register
	pub cir: Register,

	// 0x0C: peripheral reset register
	pub apb2_rstr: Register,

	// 0x10: peripheral reset register
	pub apb1_rstr: Register,

	// 0x14: peripheral clock enable register
	pub ahb_enr: Register,

	// 0x18: peripheral clock enable register
	pub apb2_enr: Register,

    // 0x1C: peripheral clock enable register
    pub apb1_enr: Register,

}

pub enum Peripheral {
    Timer2,
    Timer3,
    Timer4,
    Timer5, 
    Timer6,
    Timer7,
    Reserved1B6,
    Reserved1B7,

    Reserved1B8,
    Reserved1B9,
    Reserved1B10,
    WindowWatchdog,
    Reserved1B12,
    Reserved1B13,
    SPI2,
    SPI3,

    Reserved1B16,
    USART2,
    USART3,
    UART4,
    UART5,
    I2C1,
    I2C2,
    USB,

    Reserved1B24,
    CAN,
    Reserved1B26,
    BKP,
    PWR,
    DAC,
    Reserved1B30,
    Reserved1B31,

    // APB 2 register
    AlternateFunctionIO,
    Reserved2B1,
    IOPortA,
    IOPortB,
    IOPortC,
    IOPortD,
    IOPortE,
    IOPortF,

    IOPortG,
    ADC1,
    ADC2,
    Timer1,
    SPI1,
    Timer8,
    USART,
    ADC3,

    Reserved2B16,
    Reserved2B17,
    Reserved2B18,
    Timer9,
    Timer10,
    Timer11
}


pub enum PLLSource {
    HighSpeedInternalDiv2,
    HighSpeedExternal,
}

pub enum SystemClockSwitch {
    HighSpeedInternal,
    HighSpeedExternal,
    PLLOutput
}

pub enum AHBPrescaler {
    NoDiv = 0,
    Div2 = 0b1000,
    Div4 = 0b1001,
    Div8 = 0b1010,
    Div16 = 0b1011,
    Div64 = 0b1100,
    Div128 = 0b1101,
    Div256 = 0b1110,
    Div512 = 0b1111,
}

pub enum APBPrescaler {
    NoDiv = 0,
    Div2 = 0b100,
    Div4 = 0b101,
    Div8 = 0b110,
    Div16 = 0b111
}

pub enum PLLMultiplier {
    Mult4 = 0b10,
    Mult5 = 0b11,
    Mult6 = 0b100,
    Mult7 = 0b101,
    Mult8 = 0b110,
    Mult9 = 0b111,
    Mult6D5 = 0b1101,
}

impl RCCRegisters {

    pub fn set_apb_1_prescaler(&mut self, value : APBPrescaler) {
        let value = value as u32;
        unsafe { write_volatile(&mut(self.cfgr), 
                                common::replace(self.cfgr, value, 0b111, 8)) };
    }

    pub fn set_apb_2_prescaler(&mut self, value : APBPrescaler) {
        let value = value as u32;
        unsafe { write_volatile(&mut(self.cfgr), 
                                common::replace(self.cfgr, value, 0b111, 11)) };
    }

    pub fn set_ahb_prescaler(&mut self, value : AHBPrescaler) {
        let value = value as u32;
        unsafe { write_volatile(&mut(self.cfgr), 
                                common::replace(self.cfgr, value, 0b1111, 4)) };
    }

    pub fn set_pll_source(&mut self, value : PLLSource) {
        unsafe { write_volatile(&mut(self.cfgr), match value {
            PLLSource::HighSpeedInternalDiv2 => common::clear(self.cfgr, 16),
            PLLSource::HighSpeedExternal => common::set(self.cfgr, 16),
        }) };
    }

    pub fn set_system_clock(&mut self, value : SystemClockSwitch) {
        let value = value as u32;
        unsafe { write_volatile(&mut(self.cfgr), 
                                common::replace(self.cfgr, value, 0b1111, 0)) };
    }

    pub fn set_pll_multiplier(&mut self, value : PLLMultiplier) {
        let value = value as u32;
        unsafe { write_volatile(&mut(self.cfgr), 
                    common::replace(self.cfgr, value, 0b1111, 18)) };
    }

    write_bit_fns!(enable_hse, disable_hse, cr, 16);
    read_bit_fns!(get_hse_ready, cr, 17);
    read_bit_fns!(get_pll_ready, cr, 25);
    write_bit_fns!(enable_pll, disable_pll, cr, 24);
    
    pub fn enable_peripheral(&mut self, periph: Peripheral) {
        let mut periph = periph as u32;
        let mut port = &mut(self.apb1_enr);
        if periph >= 32 {
            periph = periph - 32;
            port = &mut(self.apb2_enr);
        } 
        unsafe { write_volatile(port, common::set(*port, periph)) };
    }
}

