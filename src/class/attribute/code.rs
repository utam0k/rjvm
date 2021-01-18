use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::attribute::Attribute;
use crate::class::constant_pool_table::Utf8Table;
use crate::class::ReaderResult;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Iconst5,
    Ldc(u8),
    Iload1,
    Aload0,
    Istore1,
    Return,
    GetStatic(u8, u8),
    InvokeVirtual(u8, u8),
    Invokespecial(u8, u8),
}

#[derive(Clone)]
struct Exception {
    start_pc: u16,
    end_pc: u16,
    handle_pc: u16,
    catch_type: u16,
}

impl Exception {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let start_pc = rdr.read_u16::<BigEndian>()?;
        let end_pc = rdr.read_u16::<BigEndian>()?;
        let handle_pc = rdr.read_u16::<BigEndian>()?;
        let catch_type = rdr.read_u16::<BigEndian>()?;

        Ok((
            Self {
                start_pc,
                end_pc,
                handle_pc,
                catch_type,
            },
            rdr,
        ))
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "start_pc: {}, end_pc: {}, handle_pc: {}, catch_type: {}",
            self.start_pc, self.end_pc, self.handle_pc, self.catch_type
        )?;
        Ok(())
    }
}

#[derive(Clone)]
// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.3
#[repr(C)]
pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    pub code_length: u32,
    pub instructions: Vec<Instruction>,
    exception_table_length: u16,
    exception_table: Vec<Exception>,
    attributes_count: u16,
    attribute_info: Vec<Attribute>,
}

impl CodeAttribute {
    pub fn new(mut rdr: Cursor<Vec<u8>>, utf8_table: &Utf8Table) -> ReaderResult<Self> {
        let max_stack = rdr.read_u16::<BigEndian>()?;
        let max_locals = rdr.read_u16::<BigEndian>()?;
        let code_length = rdr.read_u32::<BigEndian>()?;
        let (mut code, mut rdr) =
            (0..code_length).try_fold((Vec::new(), rdr), |(mut ret, mut rdr), _i| match rdr.read_u8() {
                Ok(value) => {
                    ret.push(value);
                    Ok((ret, rdr))
                }
                Err(err) => Err(err),
            })?;

        let mut instructions: Vec<Instruction> = vec![];
        code.reverse();
        loop {
            if code.is_empty() {
                break;
            }
            let inst = match code.pop() {
                Some(0x08) => Instruction::Iconst5,
                Some(0x12) => Instruction::Ldc(code.pop().unwrap()),
                Some(0x1b) => Instruction::Iload1,
                Some(0x2a) => Instruction::Aload0,
                Some(0x3c) => Instruction::Istore1,
                Some(0xb1) => Instruction::Return,
                Some(0xb2) => Instruction::GetStatic(code.pop().unwrap(), code.pop().unwrap()),
                Some(0xb6) => Instruction::InvokeVirtual(code.pop().unwrap(), code.pop().unwrap()),
                Some(0xb7) => Instruction::Invokespecial(code.pop().unwrap(), code.pop().unwrap()),
                Some(_) => unimplemented!(),
                None => panic!(),
            };
            instructions.push(inst);
        }

        let exception_table_length = rdr.read_u16::<BigEndian>()?;
        let (exception_table, mut rdr) =
            (0..exception_table_length).try_fold((Vec::new(), rdr), |(mut ret, rdr), _i| {
                match Exception::new(rdr) {
                    Ok((exception, rdr2)) => {
                        ret.push(exception);
                        Ok((ret, rdr2))
                    }
                    Err(err) => Err(err),
                }
            })?;
        let attributes_count = rdr.read_u16::<BigEndian>()?;
        let (attribute_info, rdr) = (0..attributes_count).try_fold((Vec::new(), rdr), |(mut ret, rdr), _i| {
            match Attribute::new(rdr, utf8_table) {
                Ok((ai, rdr2)) => {
                    ret.push(ai);
                    Ok((ret, rdr2))
                }
                Err(err) => Err(err),
            }
        })?;

        Ok((
            Self {
                max_stack,
                max_locals,
                code_length,
                instructions,
                exception_table_length,
                exception_table,
                attributes_count,
                attribute_info,
            },
            rdr,
        ))
    }
}

impl fmt::Debug for CodeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t max_stack: {}, max_locals: {}", self.max_stack, self.max_locals)?;
        for at in &self.attribute_info {
            write!(f, "{}", at)?;
        }
        write!(f, "\t instructsion: ")?;
        for i in &self.instructions {
            writeln!(f, "{:#?} ", i)?;
        }
        Ok(())
    }
}
