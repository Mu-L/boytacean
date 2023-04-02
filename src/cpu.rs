use core::panic;

use crate::{
    apu::Apu,
    debugln,
    inst::{EXTENDED, INSTRUCTIONS},
    mmu::Mmu,
    pad::Pad,
    ppu::Ppu,
    timer::Timer,
};

pub const PREFIX: u8 = 0xcb;

pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    ime: bool,
    zero: bool,
    sub: bool,
    half_carry: bool,
    carry: bool,
    halted: bool,

    /// Reference to the MMU (Memory Management Unit) to be used
    /// for memory bus access operations.
    pub mmu: Mmu,

    /// Temporary counter used to control the number of cycles
    /// taken by the current or last CPU operation.
    pub cycles: u8,
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Self {
        Self {
            pc: 0x0,
            sp: 0x0,
            a: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            ime: false,
            zero: false,
            sub: false,
            half_carry: false,
            carry: false,
            halted: false,
            mmu,
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0x0;
        self.sp = 0x0;
        self.a = 0x0;
        self.b = 0x0;
        self.c = 0x0;
        self.d = 0x0;
        self.e = 0x0;
        self.h = 0x0;
        self.l = 0x0;
        self.ime = false;
        self.zero = false;
        self.sub = false;
        self.half_carry = false;
        self.carry = false;
        self.halted = false;
        self.cycles = 0;
    }

    /// Sets the CPU registers and some of the memory space to the
    /// state expected after the Game Boy boot ROM executes, using
    /// these values its possible to skip the boot loading process.
    pub fn boot(&mut self) {
        self.pc = 0x0100;
        self.sp = 0xfffe;
        self.a = 0x01;
        self.b = 0xff;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xc1;
        self.h = 0x84;
        self.l = 0x03;
        self.zero = false;
        self.sub = false;
        self.half_carry = false;
        self.carry = false;

        // updates part of the MMU state, disabling the
        // boot memory overlap and setting the LCD control
        // register to enabled (required by some ROMs)
        self.mmu.set_boot_active(false);
        self.mmu.write(0xff40, 0x91);
    }

    pub fn clock(&mut self) -> u8 {
        // gathers the PC (program counter) reference that
        // is going to be used in the fetching phase
        let pc = self.pc;

        #[cfg(feature = "debug")]
        if pc >= 0x8000 && pc < 0x9fff {
            panic!("Invalid PC area at 0x{:04x}", pc);
        }

        // @TODO this is so bad, need to improve this by an order
        // of magnitude, to be able to have better performance
        if self.halted
            && (((self.mmu.ie & 0x01 == 0x01) && self.mmu.ppu().int_vblank())
                || ((self.mmu.ie & 0x02 == 0x02) && self.mmu.ppu().int_stat())
                || ((self.mmu.ie & 0x04 == 0x04) && self.mmu.timer().int_tima())
                || ((self.mmu.ie & 0x10 == 0x10) && self.mmu.pad().int_pad()))
        {
            self.halted = false;
        }

        if self.ime {
            // @TODO aggregate all of this interrupts in the MMU, as there's
            // a lot of redundant code involved in here which complicates the
            // readability and maybe performance of this code
            if (self.mmu.ie & 0x01 == 0x01) && self.mmu.ppu().int_vblank() {
                debugln!("Going to run V-Blank interrupt handler (0x40)");

                self.disable_int();
                self.push_word(pc);
                self.pc = 0x40;

                // acknowledges that the V-Blank interrupt has been
                // properly handled
                self.mmu.ppu().ack_vblank();

                // in case the CPU is currently halted waiting
                // for an interrupt, releases it
                if self.halted {
                    self.halted = false;
                }

                return 24;
            }
            // @TODO aggregate the handling of these interrupts
            else if (self.mmu.ie & 0x02 == 0x02) && self.mmu.ppu().int_stat() {
                debugln!("Going to run LCD STAT interrupt handler (0x48)");

                self.disable_int();
                self.push_word(pc);
                self.pc = 0x48;

                // acknowledges that the STAT interrupt has been
                // properly handled
                self.mmu.ppu().ack_stat();

                // in case the CPU is currently halted waiting
                // for an interrupt, releases it
                if self.halted {
                    self.halted = false;
                }

                return 24;
            }
            // @TODO aggregate the handling of these interrupts
            else if (self.mmu.ie & 0x04 == 0x04) && self.mmu.timer().int_tima() {
                debugln!("Going to run Timer interrupt handler (0x50)");

                self.disable_int();
                self.push_word(pc);
                self.pc = 0x50;

                // acknowledges that the timer interrupt has been
                // properly handled
                self.mmu.timer().ack_tima();

                // in case the CPU is currently halted waiting
                // for an interrupt, releases it
                if self.halted {
                    self.halted = false;
                }

                return 24;
            }
            // @TODO aggregate the handling of these interrupts
            else if (self.mmu.ie & 0x10 == 0x10) && self.mmu.pad().int_pad() {
                debugln!("Going to run JoyPad interrupt handler (0x60)");

                self.disable_int();
                self.push_word(pc);
                self.pc = 0x60;

                // acknowledges that the pad interrupt has been
                // properly handled
                self.mmu.pad().ack_pad();

                // in case the CPU is currently halted waiting
                // for an interrupt, releases it
                if self.halted {
                    self.halted = false;
                }

                return 24;
            }
        }

        // in case the CPU is currently in the halted state
        // returns the control flow immediately with the associated
        // number of cycles estimated for the halted execution
        if self.halted {
            return 4;
        }

        // fetches the current instruction and increments
        // the PC (program counter) accordingly
        let mut opcode = self.mmu.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        let is_prefix = opcode == PREFIX;
        let inst: &(fn(&mut Cpu), u8, &str);

        if is_prefix {
            opcode = self.mmu.read(self.pc);
            self.pc = self.pc.wrapping_add(1);
            inst = &EXTENDED[opcode as usize];
        } else {
            inst = &INSTRUCTIONS[opcode as usize];
        }

        let (inst_fn, inst_time, inst_str) = inst;

        if *inst_str == "! UNIMP !" || *inst_str == "HALT" {
            if *inst_str == "HALT" {
                debugln!("HALT with IE=0x{:02x} IME={}", self.mmu.ie, self.ime);
            }
            debugln!(
                "{}\t(0x{:02x})\t${:04x} {}",
                inst_str,
                opcode,
                pc,
                is_prefix
            );
        }

        // calls the current instruction and increments the number of
        // cycles executed by the instruction time of the instruction
        // that has just been executed
        self.cycles = 0;
        inst_fn(self);
        self.cycles = self.cycles.wrapping_add(*inst_time);

        // returns the number of cycles that the operation
        // that has been executed has taken
        self.cycles
    }

    #[inline(always)]
    pub fn mmu(&mut self) -> &mut Mmu {
        &mut self.mmu
    }

    #[inline(always)]
    pub fn mmu_i(&self) -> &Mmu {
        &self.mmu
    }

    #[inline(always)]
    pub fn ppu(&mut self) -> &mut Ppu {
        self.mmu().ppu()
    }

    #[inline(always)]
    pub fn apu(&mut self) -> &mut Apu {
        self.mmu().apu()
    }

    #[inline(always)]
    pub fn apu_i(&self) -> &Apu {
        self.mmu_i().apu_i()
    }

    #[inline(always)]
    pub fn pad(&mut self) -> &mut Pad {
        self.mmu().pad()
    }

    #[inline(always)]
    pub fn timer(&mut self) -> &mut Timer {
        self.mmu().timer()
    }

    #[inline(always)]
    pub fn halted(&self) -> bool {
        self.halted
    }

    #[inline(always)]
    pub fn cycles(&self) -> u8 {
        self.cycles
    }

    #[inline(always)]
    pub fn pc(&self) -> u16 {
        self.pc
    }

    #[inline(always)]
    pub fn sp(&self) -> u16 {
        self.sp
    }

    #[inline(always)]
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f() as u16
    }

    #[inline(always)]
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    #[inline(always)]
    pub fn f(&self) -> u8 {
        let mut f = 0x0u8;
        if self.zero {
            f |= 0x80;
        }
        if self.sub {
            f |= 0x40;
        }
        if self.half_carry {
            f |= 0x20;
        }
        if self.carry {
            f |= 0x10;
        }
        f
    }

    #[inline(always)]
    pub fn set_f(&mut self, value: u8) {
        self.zero = value & 0x80 == 0x80;
        self.sub = value & 0x40 == 0x40;
        self.half_carry = value & 0x20 == 0x20;
        self.carry = value & 0x10 == 0x10;
    }

    #[inline(always)]
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.set_f(value as u8);
    }

    #[inline(always)]
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    #[inline(always)]
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    #[inline(always)]
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    #[inline(always)]
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    #[inline(always)]
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    #[inline(always)]
    pub fn read_u8(&mut self) -> u8 {
        let byte = self.mmu.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    #[inline(always)]
    pub fn read_u16(&mut self) -> u16 {
        let byte1 = self.read_u8();
        let byte2 = self.read_u8();

        byte1 as u16 | ((byte2 as u16) << 8)
    }

    #[inline(always)]
    pub fn push_byte(&mut self, byte: u8) {
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write(self.sp, byte);
    }

    #[inline(always)]
    pub fn push_word(&mut self, word: u16) {
        self.push_byte((word >> 8) as u8);
        self.push_byte(word as u8);
    }

    #[inline(always)]
    pub fn pop_byte(&mut self) -> u8 {
        let byte = self.mmu.read(self.sp);
        self.sp = self.sp.wrapping_add(1);
        byte
    }

    #[inline(always)]
    pub fn pop_word(&mut self) -> u16 {
        self.pop_byte() as u16 | ((self.pop_byte() as u16) << 8)
    }

    #[inline(always)]
    pub fn get_zero(&self) -> bool {
        self.zero
    }

    #[inline(always)]
    pub fn set_zero(&mut self, value: bool) {
        self.zero = value
    }

    #[inline(always)]
    pub fn get_sub(&self) -> bool {
        self.sub
    }

    #[inline(always)]
    pub fn set_sub(&mut self, value: bool) {
        self.sub = value;
    }

    #[inline(always)]
    pub fn get_half_carry(&self) -> bool {
        self.half_carry
    }

    #[inline(always)]
    pub fn set_half_carry(&mut self, value: bool) {
        self.half_carry = value
    }

    #[inline(always)]
    pub fn get_carry(&self) -> bool {
        self.carry
    }

    #[inline(always)]
    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }

    #[inline(always)]
    pub fn halt(&mut self) {
        self.halted = true;
    }

    #[inline(always)]
    pub fn stop(&mut self) {
        panic!("STOP is not implemented");
    }

    #[inline(always)]
    pub fn enable_int(&mut self) {
        self.ime = true;
    }

    #[inline(always)]
    pub fn disable_int(&mut self) {
        self.ime = false;
    }
}
