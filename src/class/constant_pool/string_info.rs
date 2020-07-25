use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::ReaderResult;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.3
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StringInfo {
    pub bytes: u16,
}

impl StringInfo {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let bytes = rdr.read_u16::<BigEndian>()?;
        Ok((Self { bytes }, rdr))
    }
}

impl fmt::Display for StringInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "String   \t #{}", self.bytes)?;
        Ok(())
    }
}
