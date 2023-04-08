use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Seek, SeekFrom},
    path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};
use clap::{
    error::{ErrorKind, Result},
    ArgMatches, Error,
};
use leviathan_ir::binary::OffsetTable;
use urban_common::bus::InstructionBus;

pub struct Disassembler {
    pub index: isize,
}

impl InstructionBus for Disassembler {
    fn l0_add(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t add r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_sub(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t sub r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_mul(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t mul r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_div(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t div r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_rem(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t rem r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_divs(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t divs r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<17>(insn, 10)
        );
    }

    fn l0_rems(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t rems r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<17>(insn, 10)
        );
    }

    fn l0_ldr(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ldr r{} [0x{:08x}]",
            self.index,
            reg(insn, 0),
            self.index + signed_immediate::<22>(insn, 5) as isize * 4
        );
    }

    fn l0_str(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t str [0x{:08x}] r{}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_mov(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t mov r{} {}",
            self.index,
            reg(insn, 0),
            immediate::<22>(insn, 5)
        );
    }

    fn l0_movs(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t mov r{} {}",
            self.index,
            reg(insn, 0),
            signed_immediate::<22>(insn, 5)
        );
    }

    fn l0_branch(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch [0x{:08x}]",
            self.index,
            self.index + signed_immediate::<27>(insn, 0) as isize * 4
        );
    }

    fn l0_branch_l(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.l [0x{:08x}]",
            self.index,
            self.index + signed_immediate::<27>(insn, 0) as isize * 4
        );
    }

    fn l0_branch_ld(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.ld [0x{:08x}]",
            self.index,
            self.index + signed_immediate::<27>(insn, 0) as isize * 4
        );
    }

    fn l0_branch_l_ld(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.l.ld [0x{:08x}]",
            self.index,
            self.index + signed_immediate::<27>(insn, 0) as isize * 4
        );
    }

    fn l0_branch_eq(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.eq [0x{:08x}] r{}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_branch_ne(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.ne [0x{:08x}] r{}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_branch_lt(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.lt [0x{:08x}] r{}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_branch_gt(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.gt [0x{:08x}] r{}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_branch_le(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.le [0x{:08x}] r{}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_branch_ge(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.ge [0x{:08x}] r{}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_branch_zr(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.zr [0x{:08x}] {}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_branch_nz(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.nz [0x{:08x}] r{}",
            self.index,
            self.index + signed_immediate::<22>(insn, 0) as isize * 4,
            reg(insn, 22)
        );
    }

    fn l0_lea(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t lea r{} [0x{:08x}]",
            self.index,
            reg(insn, 0),
            self.index + signed_immediate::<22>(insn, 5) as isize * 4
        );
    }

    fn l1_shl(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t shl r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            immediate::<11>(insn, 10)
        );
    }

    fn l1_shr(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t shr r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            immediate::<11>(insn, 10)
        );
    }

    fn l1_shrs(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t shrs r{} r{} {}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            immediate::<11>(insn, 10)
        );
    }

    fn l1_ldr(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ldr r{} r{} ({})",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_ldrb(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ldrb r{} r{} ({})",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_ldrh(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ldrh r{} r{} ({})",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_ldrw(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ldrw r{} r{} ({})",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_str(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t str r{} r{} ({})",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_strb(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t strb r{} r{} ({})",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_strh(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t strh r{} r{} ({})",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_strw(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t strw r{} r{} ({})",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_int(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t int {}",
            self.index,
            immediate::<16>(insn, 0)
        );
    }

    fn l1_ncall(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ncall {}",
            self.index,
            immediate::<21>(insn, 0)
        );
    }

    fn l1_vcall(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t vcall {}",
            self.index,
            immediate::<21>(insn, 0)
        );
    }

    fn l2_add(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t add r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_sub(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t sub r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_mul(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t mul r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_div(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t div r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_rem(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t rem r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_divs(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t divs r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_rems(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t rems r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_addf(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t addf r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_subf(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t subf r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_mulf(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t mulf r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_divf(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t divf r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_remf(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t remf r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_and(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t and r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_or(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t or r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_xor(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t xor r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_shl(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t shl r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_shr(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t shr r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_shrs(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t shrs r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_cmp(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t cmp r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_cmps(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t cmps r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_cmpf(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t cmpf r{} r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l3_not(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t not r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5)
        );
    }

    fn l3_mov(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t mov r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5)
        );
    }

    fn l3_fti(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t fti r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5)
        );
    }

    fn l3_itf(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t itf r{} r{}",
            self.index,
            reg(insn, 0),
            reg(insn, 5)
        );
    }

    fn l4_branch(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_l(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.l r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_ld(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.ld r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_l_ld(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.l.ld r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_bo(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.bo r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_l_bo(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.l.bo r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_ld_bo(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.ld.bo r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_bo_ld(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.bo.ld r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_bo_ld_bo(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.bo.ld.bo r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_l_ld_bo(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.l.ld.bo r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_l_bo_ld(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.l.bo.ld r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_branch_l_bo_ld_bo(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t branch.l.bo.ld.bo r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_ncall(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ncall r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_vcall(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t vcall r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_ldbo(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ldbo r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l4_ldpc(&mut self, insn: u32) {
        println!(
            " {:>08x}:\t0x{insn:08x}\t ldpc r{}",
            self.index,
            reg(insn, 0)
        );
    }

    fn l5_nop(&mut self, insn: u32) {
        println!(" {:>08x}:\t0x{insn:08x}\t nop", self.index);
    }

    fn l5_halt(&mut self, insn: u32) {
        println!(" {:>08x}:\t0x{insn:08x}\t halt", self.index);
    }

    fn l5_ret(&mut self, insn: u32) {
        println!(" {:>08x}:\t0x{insn:08x}\t ret", self.index);
    }

    fn unknown(&mut self, insn: u32) {
        println!(" {:>08x}:\t0x{insn:08x}\t <unknown>", self.index);
    }
}

pub fn disasm(matches: &ArgMatches) -> Result<()> {
    let file: &PathBuf = matches.get_one("FILE").unwrap();
    let mut offsets = if let Some(file) = matches.get_one::<PathBuf>("OFFSETS") {
        let offset_source = fs::read_to_string(file)?;
        let OffsetTable::OffsetKey { table } = OffsetTable::read_offset_key(&offset_source)? else {
            unreachable!()
        };
        table
    } else {
        HashMap::with_capacity(0)
    };
    let file = File::open(file);
    let Ok(mut file) = file else {
        return Err(Error::raw(ErrorKind::Io, file.unwrap_err()));
    };
    file.seek(SeekFrom::Start(8))?;
    offsets.insert(
        file.read_u64::<LittleEndian>()? as usize,
        "main".to_string(),
    );
    let mut disasm = Disassembler { index: 0 };
    while let Ok(insn) = file.read_u32::<LittleEndian>() {
        if let Some(name) = offsets.get(&(disasm.index as usize)) {
            println!("<{name}>:");
        }
        disasm.process(insn);
        disasm.index += 4;
    }
    Ok(())
}

fn reg(insn: u32, bit_pos: usize) -> usize {
    (insn as usize >> bit_pos) & 0x1F
}

fn immediate<const BITS: usize>(insn: u32, bit_pos: usize) -> u64 {
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
