use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::attribute::code::CodeAttribute;
use crate::class::attribute::Attribute;
use crate::class::constant_pool_table::Utf8Table;
use crate::class::ReaderResult;

#[derive(Debug)]
pub struct NotFoundUtf8 {
    index: u16,
    utf8_table: Utf8Table,
}

impl NotFoundUtf8 {
    pub fn new(index: u16, utf8_table: Utf8Table) -> Self {
        Self { index, utf8_table }
    }
}

impl std::error::Error for NotFoundUtf8 {}

impl fmt::Display for NotFoundUtf8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "index: {}", self.index)?;
        write!(f, "Table: {:?}", self.utf8_table)?;
        Ok(())
    }
}

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.6
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MethodInfo {
    access_flags: u16,
    pub name: String,
    descriptor: String,
    attributes_count: u16,
    attribute_info: Vec<Attribute>,
}

impl MethodInfo {
    pub fn new(mut rdr: Cursor<Vec<u8>>, utf8_table: &Utf8Table) -> ReaderResult<Self> {
        let access_flags = rdr.read_u16::<BigEndian>()?;

        let name_index = rdr.read_u16::<BigEndian>()?;
        let name = utf8_table
            .get(&name_index)
            .ok_or_else(|| NotFoundUtf8::new(name_index, utf8_table.clone()))?
            .to_string();

        let descriptor_index = rdr.read_u16::<BigEndian>()?;
        let descriptor = utf8_table
            .get(&descriptor_index)
            .ok_or_else(|| NotFoundUtf8::new(descriptor_index, utf8_table.clone()))?
            .to_string();
        let attributes_count = rdr.read_u16::<BigEndian>()?;
        let (attribute_info, rdr) = (0..attributes_count).try_fold((Vec::new(), rdr), |(mut ret, rdr), _i| {
            match Attribute::new(rdr, &utf8_table) {
                Ok((ai, rdr2)) => {
                    ret.push(ai);
                    Ok((ret, rdr2))
                }
                Err(err) => Err(err),
            }
        })?;

        Ok((
            Self {
                access_flags,
                name,
                descriptor,
                attributes_count,
                attribute_info,
            },
            rdr,
        ))
    }

    pub fn code_attribute(&self) -> Vec<&CodeAttribute> {
        self.attribute_info
            .iter()
            .flat_map(|attr| match attr {
                Attribute::Code(code_attr) => Some(code_attr),
                _ => None,
            })
            .collect()
    }
}

impl fmt::Display for MethodInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\t name:             {}", self.name)?;
        writeln!(f, "\t access_flags:     {}", self.access_flags)?;
        writeln!(f, "\t descriptor:       {}", self.descriptor)?;
        writeln!(f, "\t attributes_count: {}", self.attributes_count)?;
        for ai in &self.attribute_info {
            write!(f, "{}", ai)?;
        }
        Ok(())
    }
}
