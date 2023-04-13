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
use yansi::{Color, Paint};

struct Disassembler {
    index: isize,
    offsets: HashMap<usize, (char, String)>,
}

impl Disassembler {
    fn addr(&self, offset: isize) -> String {
        let addr = (self.index + offset) as usize;
        if let Some((_, name)) = self.offsets.get(&addr) {
            return name.clone();
        }
        format!("0x{addr:08x}")
    }
}

impl InstructionBus for Disassembler {
    fn l0_add(&mut self, insn: u32) {
        println!(
            "add r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_sub(&mut self, insn: u32) {
        println!(
            "sub r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_mul(&mut self, insn: u32) {
        println!(
            "mul r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_div(&mut self, insn: u32) {
        println!(
            "div r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_rem(&mut self, insn: u32) {
        println!(
            "rem r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            immediate::<17>(insn, 10)
        );
    }

    fn l0_divs(&mut self, insn: u32) {
        println!(
            "divs r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<17>(insn, 10)
        );
    }

    fn l0_rems(&mut self, insn: u32) {
        println!(
            "rems r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<17>(insn, 10)
        );
    }

    fn l0_ldr(&mut self, insn: u32) {
        println!(
            "ldr r{} [{}]",
            reg(insn, 0),
            self.addr(signed_immediate::<22>(insn, 5) as isize * 4)
        );
    }

    fn l0_str(&mut self, insn: u32) {
        println!(
            "str [{}] r{}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_mov(&mut self, insn: u32) {
        println!("mov r{} {}", reg(insn, 0), immediate::<22>(insn, 5));
    }

    fn l0_movs(&mut self, insn: u32) {
        println!("mov r{} {}", reg(insn, 0), signed_immediate::<22>(insn, 5));
    }

    fn l0_branch(&mut self, insn: u32) {
        println!(
            "branch [{}]",
            self.addr(signed_immediate::<27>(insn, 0) as isize * 4)
        );
    }

    fn l0_branch_l(&mut self, insn: u32) {
        println!(
            "branch.l [{}]",
            self.addr(signed_immediate::<27>(insn, 0) as isize * 4)
        );
    }

    fn l0_branch_ld(&mut self, insn: u32) {
        println!(
            "branch.ld [{}]",
            self.addr(signed_immediate::<27>(insn, 0) as isize * 4)
        );
    }

    fn l0_branch_l_ld(&mut self, insn: u32) {
        println!(
            "branch.l.ld [{}]",
            self.addr(signed_immediate::<27>(insn, 0) as isize * 4)
        );
    }

    fn l0_branch_eq(&mut self, insn: u32) {
        println!(
            "branch.eq [{}] r{}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_branch_ne(&mut self, insn: u32) {
        println!(
            "branch.ne [{}] r{}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_branch_lt(&mut self, insn: u32) {
        println!(
            "branch.lt [{}] r{}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_branch_gt(&mut self, insn: u32) {
        println!(
            "branch.gt [{}] r{}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_branch_le(&mut self, insn: u32) {
        println!(
            "branch.le [{}] r{}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_branch_ge(&mut self, insn: u32) {
        println!(
            "branch.ge [{}] r{}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_branch_zr(&mut self, insn: u32) {
        println!(
            "branch.zr [{}] {}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_branch_nz(&mut self, insn: u32) {
        println!(
            "branch.nz [{}] r{}",
            self.addr(signed_immediate::<22>(insn, 0) as isize * 4),
            reg(insn, 22)
        );
    }

    fn l0_lea(&mut self, insn: u32) {
        println!(
            "lea r{} [{}]",
            reg(insn, 0),
            self.addr(signed_immediate::<22>(insn, 5) as isize * 4)
        );
    }

    fn l1_shl(&mut self, insn: u32) {
        println!(
            "shl r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            immediate::<11>(insn, 10)
        );
    }

    fn l1_shr(&mut self, insn: u32) {
        println!(
            "shr r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            immediate::<11>(insn, 10)
        );
    }

    fn l1_shrs(&mut self, insn: u32) {
        println!(
            "shrs r{} r{} {}",
            reg(insn, 0),
            reg(insn, 5),
            immediate::<11>(insn, 10)
        );
    }

    fn l1_ldr(&mut self, insn: u32) {
        println!(
            "ldr r{} r{} ({})",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_ldrb(&mut self, insn: u32) {
        println!(
            "ldrb r{} r{} ({})",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_ldrh(&mut self, insn: u32) {
        println!(
            "ldrh r{} r{} ({})",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_ldrw(&mut self, insn: u32) {
        println!(
            "ldrw r{} r{} ({})",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_str(&mut self, insn: u32) {
        println!(
            "str r{} r{} ({})",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_strb(&mut self, insn: u32) {
        println!(
            "strb r{} r{} ({})",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_strh(&mut self, insn: u32) {
        println!(
            "strh r{} r{} ({})",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_strw(&mut self, insn: u32) {
        println!(
            "strw r{} r{} ({})",
            reg(insn, 0),
            reg(insn, 5),
            signed_immediate::<11>(insn, 10)
        );
    }

    fn l1_int(&mut self, insn: u32) {
        println!("int {}", immediate::<16>(insn, 0));
    }

    fn l1_ncall(&mut self, insn: u32) {
        println!("ncall {}", immediate::<21>(insn, 0));
    }

    fn l1_vcall(&mut self, insn: u32) {
        println!("vcall {}", immediate::<21>(insn, 0));
    }

    fn l2_add(&mut self, insn: u32) {
        println!("add r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_sub(&mut self, insn: u32) {
        println!("sub r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_mul(&mut self, insn: u32) {
        println!("mul r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_div(&mut self, insn: u32) {
        println!("div r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_rem(&mut self, insn: u32) {
        println!("rem r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_divs(&mut self, insn: u32) {
        println!(
            "divs r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_rems(&mut self, insn: u32) {
        println!(
            "rems r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_addf(&mut self, insn: u32) {
        println!(
            "addf r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_subf(&mut self, insn: u32) {
        println!(
            "subf r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_mulf(&mut self, insn: u32) {
        println!(
            "mulf r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_divf(&mut self, insn: u32) {
        println!(
            "divf r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_remf(&mut self, insn: u32) {
        println!(
            "remf r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_and(&mut self, insn: u32) {
        println!("and r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_or(&mut self, insn: u32) {
        println!("or r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_xor(&mut self, insn: u32) {
        println!("xor r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_shl(&mut self, insn: u32) {
        println!("shl r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_shr(&mut self, insn: u32) {
        println!("shr r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_shrs(&mut self, insn: u32) {
        println!(
            "shrs r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_cmp(&mut self, insn: u32) {
        println!("cmp r{} r{} r{}", reg(insn, 0), reg(insn, 5), reg(insn, 10));
    }

    fn l2_cmps(&mut self, insn: u32) {
        println!(
            "cmps r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l2_cmpf(&mut self, insn: u32) {
        println!(
            "cmpf r{} r{} r{}",
            reg(insn, 0),
            reg(insn, 5),
            reg(insn, 10)
        );
    }

    fn l3_not(&mut self, insn: u32) {
        println!("not r{} r{}", reg(insn, 0), reg(insn, 5));
    }

    fn l3_mov(&mut self, insn: u32) {
        println!("mov r{} r{}", reg(insn, 0), reg(insn, 5));
    }

    fn l3_fti(&mut self, insn: u32) {
        println!("fti r{} r{}", reg(insn, 0), reg(insn, 5));
    }

    fn l3_itf(&mut self, insn: u32) {
        println!("itf r{} r{}", reg(insn, 0), reg(insn, 5));
    }

    fn l4_branch(&mut self, insn: u32) {
        println!("branch r{}", reg(insn, 0));
    }

    fn l4_branch_l(&mut self, insn: u32) {
        println!("branch.l r{}", reg(insn, 0));
    }

    fn l4_branch_ld(&mut self, insn: u32) {
        println!("branch.ld r{}", reg(insn, 0));
    }

    fn l4_branch_l_ld(&mut self, insn: u32) {
        println!("branch.l.ld r{}", reg(insn, 0));
    }

    fn l4_branch_bo(&mut self, insn: u32) {
        println!("branch.bo r{}", reg(insn, 0));
    }

    fn l4_branch_l_bo(&mut self, insn: u32) {
        println!("branch.l.bo r{}", reg(insn, 0));
    }

    fn l4_branch_ld_bo(&mut self, insn: u32) {
        println!("branch.ld.bo r{}", reg(insn, 0));
    }

    fn l4_branch_bo_ld(&mut self, insn: u32) {
        println!("branch.bo.ld r{}", reg(insn, 0));
    }

    fn l4_branch_bo_ld_bo(&mut self, insn: u32) {
        println!("branch.bo.ld.bo r{}", reg(insn, 0));
    }

    fn l4_branch_l_ld_bo(&mut self, insn: u32) {
        println!("branch.l.ld.bo r{}", reg(insn, 0));
    }

    fn l4_branch_l_bo_ld(&mut self, insn: u32) {
        println!("branch.l.bo.ld r{}", reg(insn, 0));
    }

    fn l4_branch_l_bo_ld_bo(&mut self, insn: u32) {
        println!("branch.l.bo.ld.bo r{}", reg(insn, 0));
    }

    fn l4_ncall(&mut self, insn: u32) {
        println!("ncall r{}", reg(insn, 0));
    }

    fn l4_vcall(&mut self, insn: u32) {
        println!("vcall r{}", reg(insn, 0));
    }

    fn l4_ldbo(&mut self, insn: u32) {
        println!("ldbo r{}", reg(insn, 0));
    }

    fn l4_ldpc(&mut self, insn: u32) {
        println!("ldpc r{}", reg(insn, 0));
    }

    fn l4_zero(&mut self, insn: u32) {
        println!("zero r{}", reg(insn, 0));
    }

    fn l4_dbg(&mut self, insn: u32) {
        println!("dbg r{}", reg(insn, 0));
    }

    fn l4_inc(&mut self, insn: u32) {
        println!("inc r{}", reg(insn, 0));
    }

    fn l5_nop(&mut self, _insn: u32) {
        println!("nop");
    }

    fn l5_halt(&mut self, _insn: u32) {
        println!("halt");
    }

    fn l5_ret(&mut self, _insn: u32) {
        println!("ret");
    }

    fn unknown(&mut self, _insn: u32) {
        println!("<unknown>");
    }
}

pub fn disasm(matches: &ArgMatches) -> Result<()> {
    let file: &PathBuf = matches.get_one("FILE").unwrap();
    let offsets = if let Some(file) = matches.get_one::<PathBuf>("OFFSETS") {
        let offset_source = fs::read_to_string(file)?;
        let OffsetTable { table } = OffsetTable::read_offset_key(&offset_source)?;
        table
    } else {
        HashMap::with_capacity(0)
    };
    let file = File::open(file);
    let Ok(mut file) = file else {
        return Err(Error::raw(ErrorKind::Io, file.unwrap_err()));
    };
    file.seek(SeekFrom::Start(8))?;
    let entrypoint = file.read_u64::<LittleEndian>()? as usize;
    let mut disasm = Disassembler { index: 0, offsets };
    let mut ac = ' ';
    while let Ok(insn) = file.read_u32::<LittleEndian>() {
        if disasm.index == entrypoint as isize {
            println!("entrypoint:");
        }
        if let Some((c, name)) = disasm.offsets.get(&(disasm.index as usize)) {
            ac = *c;
            match c {
                's' => {
                    println!("{}", Paint::new(format!("static {name}:")).fg(Color::Cyan));
                }
                'f' => {
                    println!("{}", Paint::new(format!("fun {name}:")).fg(Color::Yellow));
                }
                'l' => {
                    println!(
                        "{}",
                        Paint::new(format!("locals {name}:")).fg(Color::Magenta)
                    );
                }
                _ => {
                    println!("{}", Paint::new(format!("<{name}>:")).fg(Color::Red));
                }
            }
        }
        match ac {
            's' => {
                println!(
                    "{} {:>08x}:\t{:02x} {:02x} {:02x} {:02x}",
                    Paint::new("|").fg(Color::Cyan),
                    disasm.index,
                    (insn >> 24) & 0xFF,
                    (insn >> 16) & 0xFF,
                    (insn >> 8) & 0xFF,
                    insn & 0xFF
                );
            }
            'f' => {
                print!(
                    "{} {:>08x}:\t{:02x} {:02x} {:02x} {:02x}\t",
                    Paint::new("|").fg(Color::Yellow),
                    disasm.index,
                    (insn >> 24) & 0xFF,
                    (insn >> 16) & 0xFF,
                    (insn >> 8) & 0xFF,
                    insn & 0xFF
                );
                disasm.process(insn);
            }
            'l' => {
                println!(
                    "{} {:>08x}:\t{:02x} {:02x} {:02x} {:02x}",
                    Paint::new("|").fg(Color::Magenta),
                    disasm.index,
                    (insn >> 24) & 0xFF,
                    (insn >> 16) & 0xFF,
                    (insn >> 8) & 0xFF,
                    insn & 0xFF
                );
            }
            _ => {
                print!(
                    "{} {:>08x}:\t{:02x} {:02x} {:02x} {:02x}\t",
                    Paint::new("|").fg(Color::Red),
                    disasm.index,
                    (insn >> 24) & 0xFF,
                    (insn >> 16) & 0xFF,
                    (insn >> 8) & 0xFF,
                    insn & 0xFF
                );
                disasm.process(insn);
            }
        }
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
