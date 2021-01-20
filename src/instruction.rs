use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Instruction {
    Iconst5,
    Ldc(u8),
    Iload1,
    Aload0,
    Istore1,
    Return,
    GetStatic(u8, u8),
    InvokeVirtual(u8, u8),
    Invokespecial(u8, u8),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Iconst5 => write!(f, "Iconst5")?,
            Self::Ldc(arg) => write!(f, "Ldc({})", arg)?,
            Self::Iload1 => write!(f, "Iload1")?,
            Self::Aload0 => write!(f, "Aload0")?,
            Self::Istore1 => write!(f, "Istore1")?,
            Self::Return => write!(f, "Return")?,
            Self::GetStatic(arg1, arg2) => write!(f, "GetStatic({}, {})", arg1, arg2)?,
            Self::InvokeVirtual(arg1, arg2) => write!(f, "InvokeVirtual({}, {})", arg1, arg2)?,
            Self::Invokespecial(arg1, arg2) => write!(f, "Invokespecial({}, {})", arg1, arg2)?,
        };
        Ok(())
    }
}
