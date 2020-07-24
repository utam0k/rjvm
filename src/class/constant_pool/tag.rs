use std::fmt;

use num_derive::FromPrimitive;

#[derive(Debug)]
pub struct NotFoundConstantTagError {
    num: u8,
}

impl NotFoundConstantTagError {
    pub fn new(num: u8) -> Self {
        Self { num }
    }
}

impl std::error::Error for NotFoundConstantTagError {}

impl fmt::Display for NotFoundConstantTagError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConstantTag number {} is not found", self.num)?;
        Ok(())
    }
}

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4
#[derive(Clone, Copy, FromPrimitive, PartialEq)]
pub enum ConstantTag {
    Class = 7,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    String_ = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
}

impl fmt::Display for ConstantTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Class => writeln!(f, "Class")?,
            Self::Fieldref => writeln!(f, "Fieldref")?,
            Self::Methodref => writeln!(f, "Methodref")?,
            Self::InterfaceMethodref => writeln!(f, "InterfaceMethodref")?,
            Self::String_ => writeln!(f, "String")?,
            Self::Integer => writeln!(f, "Integer")?,
            Self::Float => writeln!(f, "Float")?,
            Self::Long => writeln!(f, "Long")?,
            Self::Double => writeln!(f, "Double")?,
            Self::NameAndType => writeln!(f, "NameAndType")?,
            Self::Utf8 => writeln!(f, "Utf8")?,
            Self::MethodHandle => writeln!(f, "MethodHandle")?,
            Self::MethodType => writeln!(f, "MethodType")?,
            Self::InvokeDynamic => writeln!(f, "InvokeDynamic")?,
        }
        Ok(())
    }
}
