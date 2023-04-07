use std::{
    fs::File,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};
use clap::{
    error::{ErrorKind, Result},
    ArgMatches, Error,
};
use urban_common::bus::InstructionBus;

pub struct Disassembler {
    pub index: isize,
}

impl InstructionBus for Disassembler {
    fn l0_add(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: add", self.index);
    }

    fn l0_sub(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: sub", self.index);
    }

    fn l0_mul(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: mul", self.index);
    }

    fn l0_div(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: div", self.index);
    }

    fn l0_rem(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: rem", self.index);
    }

    fn l0_divs(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: divs", self.index);
    }

    fn l0_rems(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: rems", self.index);
    }

    fn l0_ldr(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ldr", self.index);
    }

    fn l0_str(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: str", self.index);
    }

    fn l0_mov(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: mov", self.index);
    }

    fn l0_movs(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: movs", self.index);
    }

    fn l0_branch(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch 0x{:08x}",
            self.index,
            self.index + signed_immediate::<27>(insn, 0) as isize
        );
    }

    fn l0_branch_l(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.l 0x{:08x}",
            self.index,
            self.index + signed_immediate::<27>(insn, 0) as isize
        );
    }

    fn l0_branch_ld(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.ld 0x{:08x}",
            self.index,
            self.index + signed_immediate::<27>(insn, 0) as isize
        );
    }

    fn l0_branch_l_ld(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.l.ld 0x{:08x}",
            self.index,
            self.index + signed_immediate::<27>(insn, 0) as isize
        );
    }

    fn l0_branch_eq(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.eq 0x{:08x}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize
        );
    }

    fn l0_branch_ne(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.ne 0x{:08x}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize
        );
    }

    fn l0_branch_lt(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.lt 0x{:08x}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize
        );
    }

    fn l0_branch_gt(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.gt 0x{:08x}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize
        );
    }

    fn l0_branch_le(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.le 0x{:08x}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize
        );
    }

    fn l0_branch_ge(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.ge 0x{:08x}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize
        );
    }

    fn l0_branch_zr(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.zr 0x{:08x}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize
        );
    }

    fn l0_branch_nz(&mut self, insn: u32) {
        println!(
            "0x{:08x} | 0x{insn:08x}: branch.nz 0x{:08x}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize
        );
    }

    fn l0_lea(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: lea", self.index);
    }

    fn l1_shl(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: shl", self.index);
    }

    fn l1_shr(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: shr", self.index);
    }

    fn l1_shrs(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: shrs", self.index);
    }

    fn l1_ldr(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ldr", self.index);
    }

    fn l1_ldrb(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ldrb", self.index);
    }

    fn l1_ldrh(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ldrh", self.index);
    }

    fn l1_ldrw(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ldrw", self.index);
    }

    fn l1_str(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: str", self.index);
    }

    fn l1_strb(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: strb", self.index);
    }

    fn l1_strh(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: strh", self.index);
    }

    fn l1_strw(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: strw", self.index);
    }

    fn l1_int(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: int", self.index);
    }

    fn l1_ncall(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ncall", self.index);
    }

    fn l1_vcall(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: vcall", self.index);
    }

    fn l2_add(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: add", self.index);
    }

    fn l2_sub(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: sub", self.index);
    }

    fn l2_mul(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: mul", self.index);
    }

    fn l2_div(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: div", self.index);
    }

    fn l2_rem(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: rem", self.index);
    }

    fn l2_divs(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: divs", self.index);
    }

    fn l2_rems(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: rems", self.index);
    }

    fn l2_addf(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: addf", self.index);
    }

    fn l2_subf(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: subf", self.index);
    }

    fn l2_mulf(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: mulf", self.index);
    }

    fn l2_divf(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: divf", self.index);
    }

    fn l2_remf(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: remf", self.index);
    }

    fn l2_and(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: and", self.index);
    }

    fn l2_or(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: or", self.index);
    }

    fn l2_xor(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: xor", self.index);
    }

    fn l2_shl(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: shl", self.index);
    }

    fn l2_shr(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: shr", self.index);
    }

    fn l2_shrs(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: shrs", self.index);
    }

    fn l2_cmp(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: cmp", self.index);
    }

    fn l2_cmps(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: cmps", self.index);
    }

    fn l2_cmpf(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: cmpf", self.index);
    }

    fn l3_not(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: not", self.index);
    }

    fn l3_mov(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: mov", self.index);
    }

    fn l3_fti(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: fti", self.index);
    }

    fn l3_itf(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: itf", self.index);
    }

    fn l4_branch(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch", self.index);
    }

    fn l4_branch_l(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.l", self.index);
    }

    fn l4_branch_ld(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.ld", self.index);
    }

    fn l4_branch_l_ld(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.l.ld", self.index);
    }

    fn l4_branch_bo(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.bo", self.index);
    }

    fn l4_branch_l_bo(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.l.bo", self.index);
    }

    fn l4_branch_ld_bo(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.ld.bo", self.index);
    }

    fn l4_branch_bo_ld(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.bo.ld", self.index);
    }

    fn l4_branch_bo_ld_bo(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.bo.ld.bo", self.index);
    }

    fn l4_branch_l_ld_bo(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.l.ld.bo", self.index);
    }

    fn l4_branch_l_bo_ld(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.l.bo.ld", self.index);
    }

    fn l4_branch_l_bo_ld_bo(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: branch.l.bo.ld.bo", self.index);
    }

    fn l4_ncall(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ncall", self.index);
    }

    fn l4_vcall(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: vcall", self.index);
    }

    fn l4_ldbo(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ldbo", self.index);
    }

    fn l4_ldpc(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ldpc", self.index);
    }

    fn l5_nop(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: nop", self.index);
    }

    fn l5_halt(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: halt", self.index);
    }

    fn l5_ret(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: ret", self.index);
    }

    fn unknown(&mut self, insn: u32) {
        println!("0x{:08x} | 0x{insn:08x}: <unknown>", self.index);
    }
}

pub fn disasm(matches: &ArgMatches) -> Result<()> {
    let file: &PathBuf = matches.get_one("FILE").unwrap();
    let file = File::open(file);
    let Ok(mut file) = file else {
        return Err(Error::raw(ErrorKind::Io, file.unwrap_err()));
    };
    file.seek(SeekFrom::Start(16))?;
    let mut disasm = Disassembler { index: 0 };
    while let Ok(insn) = file.read_u32::<LittleEndian>() {
        disasm.process(insn);
        disasm.index += 1;
    }
    Ok(())
}

fn _reg(insn: u32, bit_pos: usize) -> usize {
    (insn as usize >> bit_pos) & 0x1F
}

fn _immediate<const BITS: usize>(insn: u32, bit_pos: usize) -> u64 {
    (insn as u64 >> bit_pos) & ((1 << BITS) - 1)
}

fn signed_immediate<const BITS: usize>(insn: u32, bit_pos: usize) -> i64 {
    let value = (insn >> bit_pos) & ((1 << BITS) - 1);
    if ((value >> (BITS - 1)) & 1) != 0 {
        (value | (!0) << BITS) as i32 as _
    } else {
        value as _
    }
}
