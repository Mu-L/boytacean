//! Game Boy specific hardware constants.

// Timer registers
pub const DIV_ADDR: u16 = 0xff04;
pub const TIMA_ADDR: u16 = 0xff05;
pub const TMA_ADDR: u16 = 0xff06;
pub const TAC_ADDR: u16 = 0xff07;
pub const IF_ADDR: u16 = 0xff0f;

// PPU registers
pub const LCDC_ADDR: u16 = 0xff40;
pub const STAT_ADDR: u16 = 0xff41;
pub const SCY_ADDR: u16 = 0xff42;
pub const SCX_ADDR: u16 = 0xff43;
pub const LY_ADDR: u16 = 0xff44;
pub const LYC_ADDR: u16 = 0xff45;

// DMA registers
pub const DMA_ADDR: u16 = 0xff46;
pub const HDMA1_ADDR: u16 = 0xff51;
pub const HDMA2_ADDR: u16 = 0xff52;
pub const HDMA3_ADDR: u16 = 0xff53;
pub const HDMA4_ADDR: u16 = 0xff54;
pub const HDMA5_ADDR: u16 = 0xff55;
