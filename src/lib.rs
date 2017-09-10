#![no_std]
#![allow(dead_code)]
#![allow(unused_macros)]
#![feature(used)]

// For documentation see: http://www.st.com/content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf

// And: http://www.st.com/resource/en/datasheet/stm32f103tb.pdf

#[macro_use] pub mod common;
pub mod gpio;
pub mod usart;
pub mod flash;
pub mod clock;
pub mod dma;
pub mod interrupts;

// other gcc functions
pub mod mem;

macro_rules! breakpoint {
	($arg:expr) => (
		unsafe { asm!("BKPT $0" : : "i"($arg) : : "volatile") }
	)
}

#[macro_export]
macro_rules! nop {
	() => (unsafe { asm!("NOP") })
}

macro_rules! register_bank {
    ($name:ident, $priv:ident, $t:ty, $addr:expr) => (
        pub const $priv : *mut $t = $addr as *mut $t;
        #[allow(non_snake_case)]
        pub fn $name() -> &'static mut $t {
            return unsafe { &mut (*$priv) };
        }
    )
}

register_bank!(FLASH, FLASH_PRIV, flash::FlashRegisters, 0x40022000);

register_bank!(RCC, RCC_PRIV, clock::RCCRegisters, 0x40021000);

register_bank!(PORT_A, PORT_A_PRIV, gpio::PortRegisters, 0x40010800);
register_bank!(PORT_B, PORT_B_PRIV, gpio::PortRegisters, 0x40010C00);
register_bank!(PORT_C, PORT_C_PRIV, gpio::PortRegisters, 0x40011000);
register_bank!(PORT_D, PORT_D_PRIV, gpio::PortRegisters, 0x40011400);
register_bank!(PORT_E, PORT_E_PRIV, gpio::PortRegisters, 0x40011800);

register_bank!(USART_1, USART_1_PRIV, usart::USARTRegisters, 0x40013800);
register_bank!(USART_2, USART_2_PRIV, usart::USARTRegisters, 0x40004400);
register_bank!(USART_3, USART_3_PRIV, usart::USARTRegisters, 0x40004800);

register_bank!(DMA, DMA_PRIV, dma::DMARegisters, 0x40020000);

