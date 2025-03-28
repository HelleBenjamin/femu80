pub const MEM_SIZE: usize = 0xFFFF;

pub enum Flags {
  CARRY = 0x01,
  PARITY = 0x04,
  AUX = 0x10,
  ZERO = 0x40,
  SIGN = 0x80
}

pub struct VirtZ80 {
  // Registers
  pub af: u16,
  pub bc: u16,
  pub de: u16,
  pub hl: u16,
  pub sp: u16,
  pub pc: u16,
  pub ix: u16,
  pub iy: u16,
  pub i: u8,
  pub r: u8,
  
  // Interrupt flip-flops
  pub iff1: bool,
  pub iff2: bool,

  // Memory
  pub mem: [u8; MEM_SIZE],
}

impl VirtZ80 {
  pub fn new() -> VirtZ80 {
    VirtZ80 {
      af: 0,
      bc: 0,
      de: 0,
      hl: 0,
      sp: 0,
      pc: 0,
      ix: 0,
      iy: 0,
      i: 0,
      r: 0,
      iff1: false,
      iff2: false,
      mem: [0; MEM_SIZE],
    }
  }
}

pub fn set_flag(z80: &mut VirtZ80, flag: u8, state: bool) {
  let _ = (z80.af & 0xff) & (flag as u16) == if state {1} else {0};
}
