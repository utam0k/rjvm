use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::ReaderResult;

#[derive(Clone)]
struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

impl LineNumber {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let start_pc = rdr.read_u16::<BigEndian>()?;
        let line_number = rdr.read_u16::<BigEndian>()?;

        Ok((Self { start_pc, line_number }, rdr))
    }
}

impl fmt::Debug for LineNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "start_pc: {}, line_number: {}", self.start_pc, self.line_number)?;
        Ok(())
    }
}

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.12
#[derive(Clone)]
#[repr(C)]
pub struct LineNumberTableAttribute {
    line_number_table_length: u16,
    line_number_table: Vec<LineNumber>,
}

impl LineNumberTableAttribute {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let line_number_table_length = rdr.read_u16::<BigEndian>()?;
        let (line_number_table, rdr) =
            (0..line_number_table_length).try_fold((Vec::new(), rdr), |(mut ret, rdr), _i| {
                match LineNumber::new(rdr) {
                    Ok((ai, rdr2)) => {
                        ret.push(ai);
                        Ok((ret, rdr2))
                    }
                    Err(err) => Err(err),
                }
            })?;

        Ok((
            Self {
                line_number_table_length,
                line_number_table,
            },
            rdr,
        ))
    }
}

impl fmt::Debug for LineNumberTableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.line_number_table {
            writeln!(f, "\t line: {}: {}", line.start_pc, line.line_number)?;
        }
        Ok(())
    }
}
