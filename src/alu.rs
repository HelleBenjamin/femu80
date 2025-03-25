
/*
    ALU Operations
    00 - ADD
    01 - SUB
    02 - ADC
    03 - SBC
    04 - AND
    05 - OR
    06 - XOR
    07 - CMP
    08 - INC
    09 - DEC

    8-bit only
    0A - RLC
    0B - RL
    0C - RRC
    0D - RR
    0E - SLA
    0F - SRA
    10 - SLL
    11 - SRL
    12 - BIT bit
    13 - RES bit
    14 - SET bit
*/

use std::{ops::{BitAnd, BitOr, BitXor}, u8};

use crate::z80core::{self};

pub enum ALUops {
    ADD,
    SUB,
    ADC,
    SBC,
    AND,
    OR,
    XOR,
    CMP,
    INC,
    DEC,
    RLC,
    RL,
    RRC,
    RR,
    SLA,
    SRA,
    SLL,
    SRL,
    BIT,
    RES,
    SET
}

pub fn update_flags16(z80: &mut z80core::VirtZ80, value: u32) {
    let mut new_flags: u8 = (z80.af & 0xFF) as u8;
    if value == 0 { new_flags |= z80core::Flags::ZERO as u8;} else {new_flags & !((z80core::Flags::ZERO as u8) as u8)}

}

pub fn alu16(z80: &mut z80core::VirtZ80, dest: &mut u16, src: u16, op: u8) -> u16 {
    match op {
        // TODO: Add that all operations modify flags
        0 => dest.wrapping_add(src), // ADD
        1  => dest.wrapping_sub(src), // SUB
        2 => dest.wrapping_add(src + (if ((z80.af & 0xff) & z80core::Flags::CARRY as u16) == 0x1 { 1 } else { 0 })), // ADC
        3 => dest.wrapping_sub(src + (if ((z80.af & 0xff) & z80core::Flags::CARRY as u16) == 0x1 { 1 } else { 0 })), // SBC
        4 => BitAnd::bitand(*dest, src), // AND
        5 => BitOr::bitor(*dest, src), // OR
        6 => BitXor::bitxor(*dest, src), // XOR
        7 => {
            let res: u16 = *dest - src;
            if res == 0 {z80core::set_flag(z80,z80core::Flags::ZERO as u8, true);}
            res
        },
        8 => dest.wrapping_add(1), // INC
        9 => dest.wrapping_sub(1), // DEC

        _ => {
            panic!("Unknown 16-bit ALU operation.");
        }
    }
}

pub fn alu8(z80: &mut z80core::VirtZ80, dest: &mut u8, src: u8, op: u8) -> u8 {
    match op {
        _ => {
            panic!("Unknown 8-bit ALU operation.")
        }
    }
}