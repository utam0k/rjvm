pub mod code;
mod line_number_table;

use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::constant_pool_table::Utf8Table;
use crate::class::method::NotFoundUtf8;
use crate::class::ReaderResult;

#[derive(Debug)]
pub struct NotFoundAttribute {
    name: String,
}

impl std::error::Error for NotFoundAttribute {}

impl NotFoundAttribute {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl fmt::Display for NotFoundAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "attribute_name: {}", self.name)?;
        Ok(())
    }
}

#[derive(Clone)]
// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7
pub enum Attribute {
    Code(code::CodeAttribute),
    LineNumberTable(line_number_table::LineNumberTableAttribute),
}

impl Attribute {
    pub fn new(mut rdr: Cursor<Vec<u8>>, utf8_table: &Utf8Table) -> ReaderResult<Self> {
        let attribute_name_index = rdr.read_u16::<BigEndian>()?;
        let _attribute_length = rdr.read_u32::<BigEndian>()?;
        let attribute_name = utf8_table
            .get(&attribute_name_index)
            .ok_or_else(|| NotFoundUtf8::new(attribute_name_index, utf8_table.clone()))?
            .to_string();

        match &*attribute_name {
            "Code" => {
                let (attribute, rdr) = code::CodeAttribute::new(rdr, utf8_table)?;
                Ok((Self::Code(attribute), rdr))
            }
            "LineNumberTable" => {
                let (attribute, rdr) = line_number_table::LineNumberTableAttribute::new(rdr)?;
                Ok((Self::LineNumberTable(attribute), rdr))
            }
            name => Err(Box::new(NotFoundAttribute::new(name.to_string()))),
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Code(attribute) => write!(f, "{:?}", attribute)?,
            Self::LineNumberTable(attribute) => write!(f, "{:?}", attribute)?,
        }
        Ok(())
    }
}
