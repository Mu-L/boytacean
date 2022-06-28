use crate::mmu::Mmu;

pub const PREFIX: u8 = 0xcb;

pub const INSTRUCTIONS: [(fn(&mut Cpu), u8, &'static str); 256] = [
    // 0x0 opcodes
    (nop, 4, "NOP"),
    (ld_bc_u16, 12, "LD BC, u16"),
    (ld_mbc_a, 8, "LD [BC], A"),
    (inc_bc, 8, "INC BC"),
    (inc_b, 4, "INC B"),
    (dec_b, 4, "DEC B"),
    (ld_b_u8, 8, "LD B, u8"),
    (rlca, 4, "RLCA"),
    (ld_mu16_sp, 20, "LD [u16], SP"),
    (add_hl_bc, 8, "ADD HL, BC"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (inc_c, 4, "INC C"),
    (nop, 4, "NOP"),
    (ld_c_u8, 8, "LD C, u8"),
    (nop, 4, "NOP"),
    // 0x1 opcodes
    (nop, 4, "NOP"),
    (ld_de_u16, 12, "LD DE, u16"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (ld_a_mde, 8, "LD A, [DE]"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x2 opcodes
    (jr_nz_i8, 8, "JR NZ, i8"),
    (ld_hl_u16, 12, "LD HL, u16"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x3 opcodes
    (nop, 4, "NOP"),
    (ld_sp_u16, 12, "LD SP, u16"),
    (ld_mhld_a, 8, "LD [HL]-, A"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (ld_a_u8, 8, "LD A, u8"),
    (nop, 4, "NOP"),
    // 0x4 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x5 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x6 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x7 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x8 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x9 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0xa opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (xor_a_a, 4, "XOR A, A"),
    // 0xb opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0xc opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (call_u16, 24, "CALL u16"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0xd opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0xe opcodes
    (ld_mff00u8_a, 12, "LD [FF00+u8], A"),
    (nop, 4, "NOP"),
    (ld_mff00c_a, 8, "LD [FF00+C], A"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0xf opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
];

pub const BITWISE: [(fn(&mut Cpu), u8, &'static str); 176] = [
    // 0x0 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x1 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x2 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x3 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x4 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x5 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x6 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x7 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (bit_7_h, 8, "BIT 7, H"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x8 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0x9 opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    // 0xa opcodes
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
    (nop, 4, "NOP"),
];

pub struct Cpu {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    zero: bool,
    sub: bool,
    half_carry: bool,
    carry: bool,
    mmu: Mmu,
    ticks: u32,
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Cpu {
        Cpu {
            pc: 0x0,
            sp: 0x0,
            a: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            zero: false,
            sub: false,
            half_carry: false,
            carry: false,
            mmu: mmu,
            ticks: 0,
        }
    }

    pub fn clock(&mut self) {
        let pc = self.pc;

        // fetches the current instruction and increments
        // the PC (program counter) accordingly
        let mut opcode = self.mmu.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        let instruction: &(fn(&mut Cpu), u8, &str);

        if opcode == PREFIX {
            opcode = self.mmu.read(self.pc);
            self.pc = self.pc.wrapping_add(1);
            instruction = &BITWISE[opcode as usize];
        } else {
            instruction = &INSTRUCTIONS[opcode as usize];
        }

        let (instruction_fn, instruction_size, instruction_str) = instruction;

        println!("{}\t(0x{:02x})\t${:04x}", instruction_str, opcode, pc);

        instruction_fn(self);
        self.ticks = self.ticks.wrapping_add(*instruction_size as u32);
    }

    #[inline(always)]
    pub fn mmu(&mut self) -> &mut Mmu {
        &mut self.mmu
    }

    #[inline(always)]
    pub fn pc(&self) -> u16 {
        self.pc
    }

    #[inline(always)]
    fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f() as u16
    }

    #[inline(always)]
    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    #[inline(always)]
    fn f(&self) -> u8 {
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
    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    #[inline(always)]
    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    #[inline(always)]
    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    #[inline(always)]
    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    #[inline(always)]
    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    #[inline(always)]
    fn read_u8(&mut self) -> u8 {
        let byte = self.mmu.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    #[inline(always)]
    fn read_u16(&mut self) -> u16 {
        let byte1 = self.read_u8();
        let byte2 = self.read_u8();
        let word = byte1 as u16 | ((byte2 as u16) << 8);
        word
    }

    #[inline(always)]
    fn get_zero(&self) -> bool {
        self.zero
    }

    #[inline(always)]
    fn set_zero(&mut self, value: bool) {
        self.zero = value
    }

    #[inline(always)]
    fn get_sub(&self) -> bool {
        self.sub
    }

    #[inline(always)]
    fn set_sub(&mut self, value: bool) {
        self.sub = value;
    }

    #[inline(always)]
    fn get_half_carry(&self) -> bool {
        self.half_carry
    }

    #[inline(always)]
    fn set_half_carry(&mut self, value: bool) {
        self.half_carry = value
    }

    #[inline(always)]
    fn get_carry(&self) -> bool {
        self.carry
    }

    #[inline(always)]
    fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }
}

fn nop(_cpu: &mut Cpu) {}

fn ld_bc_u16(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.set_bc(word);
}

fn ld_mbc_a(cpu: &mut Cpu) {
    cpu.mmu.write(cpu.bc(), cpu.a);
}

fn inc_bc(cpu: &mut Cpu) {
    cpu.set_bc(cpu.bc().wrapping_add(1));
}

fn inc_b(cpu: &mut Cpu) {
    let value = cpu.b.wrapping_add(1);

    cpu.set_sub(false);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((value & 0xf) == 0xf);

    cpu.b = value;
}

fn dec_b(cpu: &mut Cpu) {
    let value = cpu.b.wrapping_sub(1);

    cpu.set_sub(true);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((value & 0xf) == 0xf);

    cpu.b = value;
}

fn ld_b_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.b = byte;
}

fn rlca(cpu: &mut Cpu) {
    let carry = cpu.a >> 7;

    cpu.a = cpu.a << 1 | carry;

    cpu.set_sub(false);
    cpu.set_zero(false);
    cpu.set_half_carry(false);
    cpu.set_carry(carry == 1);
}

fn ld_mu16_sp(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.mmu.write(word, cpu.sp as u8);
    cpu.mmu.write(word + 1, (cpu.sp >> 8) as u8);
}

fn add_hl_bc(cpu: &mut Cpu) {
    let value = add_u16_u16(cpu, cpu.hl(), cpu.bc());
    cpu.set_hl(value);
}

fn inc_c(cpu: &mut Cpu) {
    let value = cpu.c.wrapping_add(1);

    cpu.set_sub(false);
    cpu.set_zero(value == 0);
    cpu.set_half_carry((value & 0xf) == 0xf);

    cpu.c = value;
}

fn ld_c_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.c = byte;
}

fn ld_de_u16(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.set_de(word);
}

fn ld_a_mde(cpu: &mut Cpu) {
    let byte = cpu.mmu.read(cpu.de());
    cpu.a = byte;
}

fn jr_nz_i8(cpu: &mut Cpu) {
    let byte = cpu.read_u8() as i8;

    if cpu.get_zero() {
        return;
    }

    cpu.pc = (cpu.pc as i16).wrapping_add(byte as i16) as u16;
    cpu.ticks = cpu.ticks.wrapping_add(4);
}

fn ld_hl_u16(cpu: &mut Cpu) {
    let word = cpu.read_u16();
    cpu.set_hl(word);
}

fn ld_sp_u16(cpu: &mut Cpu) {
    cpu.sp = cpu.read_u16();
}

fn ld_mhld_a(cpu: &mut Cpu) {
    cpu.mmu.write(cpu.hl(), cpu.a);
    cpu.set_hl(cpu.hl().wrapping_sub(1));
}

fn ld_a_u8(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.a = byte;
}

fn xor_a_a(cpu: &mut Cpu) {
    cpu.a ^= cpu.a;

    cpu.set_sub(false);
    cpu.set_zero(cpu.a == 0);
    cpu.set_half_carry(false);
    cpu.set_carry(false);
}

fn call_u16(cpu: &mut Cpu) {
    // @todo push stack and set the current PC
    // to be able to jump
    cpu.sp += 1;
}

fn ld_mff00u8_a(cpu: &mut Cpu) {
    let byte = cpu.read_u8();
    cpu.mmu.write(0xff0c + byte as u16, cpu.a);
}

fn ld_mff00c_a(cpu: &mut Cpu) {
    cpu.mmu.write(0xff0c + cpu.c as u16, cpu.a);
}

fn bit_7_h(cpu: &mut Cpu) {
    bit_h(cpu, 7);
}

/// Helper function to test one bit in a u8.
/// Returns true if bit is 0.
fn bit_zero(val: u8, bit: u8) -> bool {
    (val & (1u8 << (bit as usize))) == 0
}

fn bit_h(cpu: &mut Cpu, bit: u8) {
    println!("{}", cpu.h);
    cpu.set_sub(false);
    cpu.set_zero(bit_zero(cpu.h, bit));
    cpu.set_half_carry(true);
}

fn add_u16_u16(cpu: &mut Cpu, first: u16, second: u16) -> u16 {
    let first = first as u32;
    let second = second as u32;
    let value = first.wrapping_add(second);

    cpu.set_sub(false);
    cpu.set_carry(value & 0x1000 == 0x1000);
    cpu.set_half_carry((first ^ second ^ value) & 0x1000 == 0x1000);

    value as u16
}
