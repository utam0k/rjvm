use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::ReaderResult;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.1
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ClassInfo {
    name_index: u16,
}

impl ClassInfo {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let name_index = rdr.read_u16::<BigEndian>()?;
        Ok((Self { name_index }, rdr))
    }
}

impl fmt::Display for ClassInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Class   \t #{}", self.name_index,)?;
        Ok(())
    }
}
