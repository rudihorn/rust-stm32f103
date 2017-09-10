use ::core::ops::*;
use ::core::ptr::{write_volatile, read_volatile};


pub trait BitValue<T> : Copy +  Shl<T,Output=T> + Shr<T,Output=T> + BitAnd<T,Output=T> + BitOr<T, Output=T> + Not<Output=T> { }
impl<T> BitValue<T> for T where T:  Copy + Shl<T,Output=T> + Shr<T,Output=T> + BitAnd<T,Output=T> + BitOr<T, Output=T> + Not<Output=T> { }


pub fn replace<T : BitValue<T>>(old : T, new : T, mask : T, offset : T) -> T {
//pub fn replace<T : Shl<T, Output=T> + BitAnd<T, Output=T> + BitOr<T, Output=T> + Not<Output=T>>(old : T, new : T, mask : T, offset : T) -> T {
//pub fn replace(old : u32, new : u32, mask : u32, offset : u32) -> u32 {
    let mask = mask << offset;
    let new = new << offset;
    return (old & !mask) | new;
}

pub fn set(old : u32, bit : u32) -> u32 {
    return old | (1u32 << bit);
}

pub fn clear(old : u32, bit : u32) -> u32 {
    return old & !(1u32 << bit);
}

#[macro_export]
macro_rules! clear_bit {
    ($reg:expr, $bit:expr) => (
        unsafe { write_volatile(&mut($reg), read_volatile(&($reg)) & !(1u32 << $bit)) }
    )
}

#[macro_export]
macro_rules! set_bit {
    ($reg:expr, $bit:expr) => (
        unsafe { write_volatile(&mut($reg), read_volatile(&($reg)) | (1u32 << $bit)) }
    )
}

#[macro_export]
macro_rules! clear_bit_by_set {
    ($reg:expr, $bit:expr) => (
        unsafe { write_volatile(&mut($reg), (1u32 << $bit)) }
    )
}

#[macro_export]
macro_rules! read_bit {
    ($reg:expr, $bit:expr) => (
        unsafe { return (read_volatile(&($reg)) >> ($bit)) & 0b1 > 0 }
    )
}


pub type Register = u32;

pub trait Enableable {
    fn enable(&'static mut self);
    fn disable(&'static mut self);
}

pub struct RegisterEnableBit {
    reg : &'static mut Register,
    bit : u32,
}

impl Enableable for RegisterEnableBit {
    fn enable(&'static mut self) { unsafe { write_volatile(self.reg, set(read_volatile(self.reg), self.bit)) }; }
    fn disable(&'static mut self) { unsafe { write_volatile(self.reg, clear(read_volatile(self.reg), self.bit)) }; }
}

#[macro_export]
macro_rules! clear_bit_fn {
    ($name_dis:ident, $reg:ident, $pin:expr) => (
        pub fn $name_dis(&mut self) {
            unsafe { write_volatile(&mut(self.$reg), common::clear(read_volatile(&(self.$reg)), $pin)) };
        }
    )
}

#[macro_export]
macro_rules! set_bit_fn {
    ($name_en:ident, $reg:ident, $pin:expr) => (
        pub fn $name_en(&mut self) {
            unsafe { write_volatile(&mut(self.$reg), common::set(read_volatile(&(self.$reg)), $pin)) };
        }
    )
}

#[macro_export]
macro_rules! read_bit_fns {
    ($name:ident, $reg:ident, $pin:expr) => (
        pub fn $name(&mut self) -> bool {
            return unsafe { (read_volatile(&(self.$reg)) >> $pin) & 0b1 } > 0;
        }
    )
}

#[macro_export]
macro_rules! read_clear_bit_fns {
    ($name_read:ident, $name_dis:ident, $reg:ident, $pin:expr) => (
        read_bit_fns!($name_read, $reg, $pin);
        clear_bit_fn!($name_dis, $reg, $pin);
    )
}

#[macro_export]
macro_rules! write_bit_fns {
    ($name_en:ident, $name_dis:ident, $reg:ident, $pin:expr) => (
        set_bit_fn!($name_en, $reg, $pin);
        clear_bit_fn!($name_dis, $reg, $pin);
    )
}

