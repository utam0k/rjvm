use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::ReaderResult;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.6
#[repr(C)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
}

impl MethodInfo {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let access_flags = rdr.read_u16::<BigEndian>()?;
        let name_index = rdr.read_u16::<BigEndian>()?;
        let descriptor_index = rdr.read_u16::<BigEndian>()?;
        let attributes_count = rdr.read_u16::<BigEndian>()?;
        Ok((
            Self {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
            },
            rdr,
        ))
    }
}

impl fmt::Display for MethodInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t access_flags:     {}", self.access_flags,)?;
        writeln!(f, "\t name_index:       {}", self.name_index,)?;
        writeln!(f, "\t descriptor_index: {}", self.descriptor_index,)?;
        writeln!(f, "\t attributes_count: {}", self.attributes_count,)?;
        Ok(())
    }
}
