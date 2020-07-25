use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::ReaderResult;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.2
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MethodrefInfo {
    class_index: u16,
    name_and_type_index: u16,
}

impl MethodrefInfo {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let class_index = rdr.read_u16::<BigEndian>()?;
        let name_and_type_index = rdr.read_u16::<BigEndian>()?;
        Ok((
            Self {
                class_index,
                name_and_type_index,
            },
            rdr,
        ))
    }
}

impl fmt::Display for MethodrefInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Methodref\t #{}.#{}", self.class_index, self.name_and_type_index)?;
        Ok(())
    }
}
