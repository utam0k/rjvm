use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::ReaderResult;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.6
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NameAndTypeInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl NameAndTypeInfo {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let name_index = rdr.read_u16::<BigEndian>()?;
        let descriptor_index = rdr.read_u16::<BigEndian>()?;
        Ok((
            Self {
                name_index,
                descriptor_index,
            },
            rdr,
        ))
    }
}

impl fmt::Display for NameAndTypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NameAndType\t #{}:#{}", self.name_index, self.descriptor_index)?;
        Ok(())
    }
}
