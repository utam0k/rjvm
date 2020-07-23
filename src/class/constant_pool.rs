mod class_info;
mod fieldref_info;
mod interface_methodref_info;
mod methodref_info;
mod name_and_type_info;
mod string_info;
mod tag;
mod utf8_info;

use std::fmt;
use std::io::Cursor;

use byteorder::ReadBytesExt;
use num_traits::FromPrimitive;

use crate::class::ReaderResult;
use class_info::ClassInfo;
use fieldref_info::FieldrefInfo;
use interface_methodref_info::InterfaceMethodrefInfo;
use methodref_info::MethodrefInfo;
use name_and_type_info::NameAndTypeInfo;
use string_info::StringInfo;
use tag::{ConstantTag, NotFoundConstantTagError};
use utf8_info::Utf8Info;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4
#[repr(C)]
pub struct ConstantPool {
    tag: ConstantTag,
    info: ConstantPoolInfo,
}

impl ConstantPool {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let tag_number = rdr.read_u8()?;
        let tag = FromPrimitive::from_u8(tag_number)
            .ok_or_else(|| NotFoundConstantTagError::new(tag_number))?;
        let (info, rdr) = ConstantPoolInfo::new(&tag, rdr)?;
        Ok((ConstantPool { tag, info }, rdr))
    }
}

impl fmt::Display for ConstantPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\t{}", self.info)?;
        Ok(())
    }
}

pub enum ConstantPoolInfo {
    ClassInfo(ClassInfo),
    FieldrefInfo(FieldrefInfo),
    MethodrefInfo(MethodrefInfo),
    InterfaceMethodrefInfo(InterfaceMethodrefInfo),
    StringInfo(StringInfo),
    NameAndTypeInfo(NameAndTypeInfo),
    Utf8Info(Utf8Info),
}

impl fmt::Display for ConstantPoolInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::ClassInfo(info) => write!(f, "{}", info)?,
            Self::FieldrefInfo(info) => write!(f, "{}", info)?,
            Self::MethodrefInfo(info) => write!(f, "{}", info)?,
            Self::InterfaceMethodrefInfo(info) => write!(f, "{}", info)?,
            Self::StringInfo(info) => write!(f, "{}", info)?,
            Self::NameAndTypeInfo(info) => write!(f, "{}", info)?,
            Self::Utf8Info(info) => write!(f, "{}", info)?,
        };
        Ok(())
    }
}

impl ConstantPoolInfo {
    pub fn new(tag: &ConstantTag, rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        Ok(match tag {
            ConstantTag::Class => {
                let (info, rdr) = ClassInfo::new(rdr)?;
                (Self::ClassInfo(info), rdr)
            }
            ConstantTag::Fieldref => {
                let (info, rdr) = FieldrefInfo::new(rdr)?;
                (Self::FieldrefInfo(info), rdr)
            }
            ConstantTag::Methodref => {
                let (info, rdr) = MethodrefInfo::new(rdr)?;
                (Self::MethodrefInfo(info), rdr)
            }
            ConstantTag::InterfaceMethodref => {
                let (info, rdr) = InterfaceMethodrefInfo::new(rdr)?;
                (Self::InterfaceMethodrefInfo(info), rdr)
            }
            ConstantTag::String_ => {
                let (info, rdr) = StringInfo::new(rdr)?;
                (Self::StringInfo(info), rdr)
            }
            ConstantTag::NameAndType => {
                let (info, rdr) = NameAndTypeInfo::new(rdr)?;
                (Self::NameAndTypeInfo(info), rdr)
            }
            ConstantTag::Utf8 => {
                let (info, rdr) = Utf8Info::new(rdr)?;
                (Self::Utf8Info(info), rdr)
            }
            _ => unimplemented!("Sorry..."),
        })
    }
}
