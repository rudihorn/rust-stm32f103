#![allow(dead_code)]

use super::common;
use super::common::{Register};
use ::core::ptr::{write_volatile, read_volatile};

pub struct DMAChannelRegister {
    // 0x00: DMA channel configuration register
    pub ccr: Register,

    // 0x04: DMA channel number of data register
    pub cndtr: Register,

    // 0x08: DMA channel peripheral address register
    pub cpar: Register,

    // 0x0C: DMA channel memory address register
    pub cmar: Register,

    // 0x10: Reserved
    pub res: Register,
}

pub enum Priority {
    Low,
    Medium,
    High,
    VeryHigh
}

pub enum MemorySize {
    Bits8,
    Bits16,
    Bits32,
}
    
pub enum DataTransferDirection {
    ReadFromPeripheral,
    ReadFromMemory
}

impl DMAChannelRegister {
    pub fn set_number_of_data(&mut self, val:u16) {
        let val = val as u32;
        unsafe { write_volatile(&mut(self.cndtr), val); }
    }

    pub fn set_peripheral_address(&mut self, reg:&Register) {
        let ptr = reg as *const Register;
        let ptr = ptr as u32;
        unsafe { write_volatile(&mut(self.cpar), ptr); }
    }

    pub fn set_memory_address(&mut self, adr : u32) {
        unsafe { write_volatile(&mut(self.cmar), adr) };
    }

    pub fn set_priority(&mut self, val : Priority) {
        let val = val as u32;
        unsafe { write_volatile(&mut(self.ccr), 
                                common::replace(self.ccr, val, 0b11, 12)) };
    }

    pub fn set_msize(&mut self, val : MemorySize) {
        let val = val as u32;
        unsafe { write_volatile(&mut(self.ccr), 
                                common::replace(self.ccr, val, 0b11, 10)) };
    }

    pub fn set_psize(&mut self, val : MemorySize) {
        let val = val as u32;
        unsafe { write_volatile(&mut(self.ccr), 
                                common::replace(self.ccr, val, 0b11, 8)) };
    }

    pub fn set_transfer_dir(&mut self, val : DataTransferDirection) {
        match val {
            DataTransferDirection::ReadFromPeripheral => clear_bit!(self.ccr, 4),
            DataTransferDirection::ReadFromMemory => set_bit!(self.ccr, 4),
        };
    }
        
    write_bit_fns!(enable, disable, ccr, 0);
    write_bit_fns!(enable_transf_int, disable_transf_int, ccr, 1);
    write_bit_fns!(enable_half_transf_int, disable_half_transf_int, ccr, 2);
    write_bit_fns!(enable_mem_incr_mode, disable_mem_incr_mode, ccr, 7);

}


pub struct DMARegisters {
    // 0x00: Interrupt status register
    pub isr: Register,

    // 0x04: DMA interrupt flag clear Register
    pub ifcr: Register,

    // DMA channels
    
    // for adc1, tim2 ch3, tim4 ch1
    pub dma1: DMAChannelRegister,
    // for usart3 tx, tim1 ch1, tim2 up, tim3 ch3, spi1 rx
    pub dma2: DMAChannelRegister,
    // for usart3 rx, tim1 ch2, tim3 ch4, tim3 up, spi1 tx
    pub dma3: DMAChannelRegister,
    // for usart1 tx, tim1 ch4, tim1 trig, tim1 com, tim4 ch2, spi i2s2_rx, i2c2_tx
    pub dma4: DMAChannelRegister,
    // for usart1 rx, tim1 up, spi i2s2_tx, tim2 ch1, tim4 ch3, i2c2 rx
    pub dma5: DMAChannelRegister,
    // for usart2 rx, tim1 ch3, tim3 ch1, tim3 trig, i2c1 tx
    pub dma6: DMAChannelRegister,
    // for usart2 tx, tim2 ch2, tim2 ch4, tim4 up, i2c1 rx
    pub dma7: DMAChannelRegister,
}

pub enum Channels {
    Channel1,
    Channel2,
    Channel3, 
    Channel4,
    Channel5, 
    Channel6, 
    Channel7,
}

impl DMARegisters {
    pub fn read_error(&mut self, chan: Channels) -> bool {
        let bit = 3 + 4 * (chan as u32);
        return read_bit!(self.isr, bit);
    }

    pub fn clear_error(&mut self, chan: Channels) {
        let bit = 3 + 4 * (chan as u32);
        clear_bit_by_set!(self.ifcr, bit); 
    }

    pub fn read_half_transfer(&mut self, chan: Channels) -> bool {
        let bit = 2 + 4 * (chan as u32);
        return read_bit!(self.isr, bit);
    }

    pub fn clear_half_transfer(&mut self, chan : Channels) {
        let bit = 2 + 4 * (chan as u32);
        clear_bit_by_set!(self.ifcr, bit);
    }

    pub fn read_transfer_complete(&mut self, chan : Channels) -> bool {
        let bit = 1 + 4 * (chan as u32);
        return read_bit!(self.isr, bit);
    }

    pub fn clear_transfer_complete(&mut self, chan : Channels) {
        let bit = 1 + 4 * (chan as u32);
        clear_bit_by_set!(self.ifcr, bit);
    }

    pub fn read_global_interrupt(&mut self, chan : Channels) -> bool {
        let bit = 4 * (chan as u32);
        return read_bit!(self.isr, bit);
    }

    pub fn clear_global_interrupt(&mut self, chan : Channels) {
        let bit = 4 * (chan as u32);
        clear_bit_by_set!(self.ifcr, bit);
    }
}

pub struct DMAWriteBuffer {
    pub channel : Option<&'static mut DMAChannelRegister>,
    pub buffer : Option<&'static mut [u8]>,
    // buffer start .. end_dma being written,
    // end_dma .. end_write to be written
    pub start : u32,
    pub end_dma : u32,
    pub end_write : u32,
}


impl DMAWriteBuffer {

    pub fn enable(&mut self) {
        self.get_channel().enable();
    }

    pub fn get_channel(&mut self) -> &mut DMAChannelRegister {
        match self.channel {
            Some(ref mut x) => return x,
            None => panic!()
        }
    }

    pub fn get_buffer(&mut self) -> &mut [u8] {
        match self.buffer {
            Some(ref mut x) => return x,
            None => panic!(),
        }
    }

    pub fn write(&mut self, val:u8) {
        // block because buffer is full
        while (self.end_write + 1 % self.get_buffer().len() as u32) == self.start { }
        
        self.get_buffer()[self.start as usize] = val;
        self.end_write = (self.end_write + 1) % self.get_buffer().len() as u32;
    }

    pub fn next_write(&mut self, val:u8) {

        self.start = self.end_dma;

        // write
        let len = if self.end_write < self.end_dma {
            self.end_dma = 0;
            self.get_buffer().len() as u32 - self.end_dma
        } else {
            self.end_dma = self.end_write;
            self.end_write - self.end_dma
        };

        let start = (&(self.get_buffer()[0]) as *const u8) as u32 + self.start;
        self.get_channel().set_memory_address(start);

        self.get_channel().set_number_of_data(len as u16);
    }
}
