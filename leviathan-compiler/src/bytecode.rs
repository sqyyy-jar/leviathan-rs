use leviathan_common::prelude::*;
use std::io::Write;

const MAGIC: &[u8; 4] = b"tvl\0";
const SECTION_FUNCTIONS: u8 = 0x08;
const SECTION_STRING_TABLE: u8 = 0x09;
const SECTION_GENERIC: u8 = 0x0F;

pub struct Bytecode {
    pub sections: Vec<BytecodeSection>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            sections: Vec::with_capacity(0),
        }
    }

    pub fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write(MAGIC)?;
        writer.write(&(self.sections.len() as u32).to_le_bytes())?;
        for section in &self.sections {
            section.write(writer)?;
        }
        Ok(())
    }
}

pub enum BytecodeSection {
    Functions(Vec<CompiledFunction>),
    StringTable(Vec<String>),
    Generic(Vec<u8>),
}

impl BytecodeSection {
    pub fn write(&self, writer: &mut impl Write) -> Result<()> {
        match self {
            BytecodeSection::Functions(functions) => {
                writer.write(&SECTION_FUNCTIONS.to_le_bytes())?;
                writer.write(&(functions.len() as u32).to_le_bytes())?;
                for function in functions {
                    function.write(writer)?;
                }
            }
            BytecodeSection::StringTable(strings) => {
                writer.write(&SECTION_STRING_TABLE.to_le_bytes())?;
                writer.write(&(strings.len() as u32).to_le_bytes())?;
                for string in strings {
                    let bytes = string.as_bytes();
                    writer.write(&(bytes.len() as u32).to_le_bytes())?;
                    writer.write(bytes)?;
                }
            }
            BytecodeSection::Generic(bytes) => {
                writer.write(&SECTION_GENERIC.to_le_bytes())?;
                writer.write(&(bytes.len() as u32).to_le_bytes())?;
                writer.write(bytes)?;
            }
        }
        Ok(())
    }
}

pub struct CompiledFunction {
    pub name_index: u32,
    pub return_type: CompiledType,
    pub arguments: Vec<CompiledType>,
    pub code: CompiledCode,
}

impl CompiledFunction {
    pub fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write(&self.name_index.to_le_bytes())?;
        self.return_type.write(writer)?;
        writer.write(&(self.arguments.len() as u32).to_le_bytes())?;
        for argument in &self.arguments {
            argument.write(writer)?;
        }
        self.code.write(writer)?;
        Ok(())
    }
}

pub struct CompiledCode {
    pub bytecode: Vec<u16>,
}

impl CompiledCode {
    pub fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write(&(self.bytecode.len() as u32).to_le_bytes())?;
        for word in &self.bytecode {
            writer.write(&word.to_le_bytes())?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum CompiledType {
    Unit = 0x00,
    Bool = 0x01,
    Int = 0x02,
    Float = 0x03,
    String = 0x4,
    Atom = 0x05,
    List = 0x06,
    Map = 0x07,
}

impl CompiledType {
    pub fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write(&(self.clone() as u8).to_le_bytes())?;
        Ok(())
    }
}
