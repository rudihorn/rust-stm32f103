

pub type IVFunction = fn ();

pub struct IVTable {
	pub main: IVFunction,
	pub nmi_handler: IVFunction,
	pub hard_fault: IVFunction,
    pub memmanage: IVFunction,
	pub bus_fault: IVFunction,
	pub usage_fault: IVFunction,
    pub reserved1: u32,
    pub svcall : IVFunction,
    pub debugmonitor: IVFunction,
    pub reserved2: u32,
    pub pendsv: IVFunction,
    pub systick: IVFunction,

    // 0 .. 4
    pub wwdg: IVFunction,
    pub pvd: IVFunction,
    pub tamper: IVFunction,
    pub rtc: IVFunction,
    pub flash: IVFunction,

    // 5 .. 9
    pub rcc: IVFunction,
    pub exti0: IVFunction,    
    pub exti1: IVFunction,
    pub exti2: IVFunction,
    pub exti3: IVFunction,

    // 10 .. 14
    pub exti4: IVFunction,
    pub dma1_channel1: IVFunction,
    pub dma1_channel2: IVFunction,
    pub dma1_channel3: IVFunction,
    pub dma1_channel4: IVFunction,

    // 15 .. 19
    pub dma1_channel5: IVFunction,
    pub dma1_channel6: IVFunction,
    pub dma1_channel7: IVFunction,
    pub adc1_2: IVFunction,
    pub can1_tx: IVFunction,

    // 20 .. 24
    pub can1_rx0: IVFunction,
    pub can1_rx1: IVFunction,
    pub can1_sce: IVFunction,
    pub exti9_5: IVFunction,
    pub tim1_brk: IVFunction,
    
    // 25 .. 29
    pub tim1_up: IVFunction,
    pub tim1_trg_com: IVFunction,
    pub tim1_cc: IVFunction,
    pub tim2: IVFunction,
    pub tim3: IVFunction,

    // 30 .. 34 
    pub tim4: IVFunction,
    pub i2c1_ev: IVFunction,
    pub i2c1_er: IVFunction,
    pub i2c2_ev: IVFunction,
    pub i2c2_er: IVFunction,

    // 35 .. 39
    pub spi: IVFunction,
    pub spi2: IVFunction,
    pub usart1: IVFunction,
    pub usart2: IVFunction,
    pub usart3: IVFunction,

    // 40 .. 44
    pub exti15_10: IVFunction,
    pub rtcalarm: IVFunction,
    pub otg_fs_wkp: IVFunction,
    pub reserved3: u32,
    pub reserved4: u32,

    // 45 .. 49
    pub reserved5: u32,
    pub reserved6: u32,
    pub reserved7: u32,
    pub reserved8: u32,
    pub reserved9: u32,

    // 50 .. 54
    pub tim5: IVFunction,
    pub spi3: IVFunction,
    pub uart4: IVFunction,
    pub uart5: IVFunction,
    pub tim6: IVFunction,

    // 55 .. 59
    pub tim7: IVFunction,
    pub dma2_channel1: IVFunction,
    pub dma2_channel2: IVFunction,
    pub dma2_channel3: IVFunction,
    pub dma2_channel4: IVFunction,

    // 60 .. 64
    pub dma2_channel5: IVFunction,
    pub eth: IVFunction,
    pub eth_wkup: IVFunction,
    pub can2_tx: IVFunction,
    pub can2_rx0: IVFunction,

    // 65 .. 67
    pub can2_rx1: IVFunction,
    pub can2_sce: IVFunction,
    pub otg_fs: IVFunction,
}

