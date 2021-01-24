use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Instruction {
    Iconst3,
    Iconst5,
    Ldc(u8),
    Iload1,
    Iload2,
    Aload0,
    Istore1,
    Istore2,
    Iadd,
    Return,
    GetStatic(u8, u8),
    InvokeVirtual(u8, u8),
    Invokespecial(u8, u8),
}

impl Instruction {
    pub fn from_codes(mut codes: Vec<u8>) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = vec![];
        loop {
            if codes.is_empty() {
                break;
            }
            let inst = match codes.pop() {
                Some(0x06) => Instruction::Iconst3,
                Some(0x08) => Instruction::Iconst5,
                Some(0x12) => Instruction::Ldc(codes.pop().unwrap()),
                Some(0x1b) => Instruction::Iload1,
                Some(0x1c) => Instruction::Iload2,
                Some(0x2a) => Instruction::Aload0,
                Some(0x3c) => Instruction::Istore1,
                Some(0x3d) => Instruction::Istore2,
                Some(0x60) => Instruction::Iadd,
                Some(0xb1) => Instruction::Return,
                Some(0xb2) => Instruction::GetStatic(codes.pop().unwrap(), codes.pop().unwrap()),
                Some(0xb6) => Instruction::InvokeVirtual(codes.pop().unwrap(), codes.pop().unwrap()),
                Some(0xb7) => Instruction::Invokespecial(codes.pop().unwrap(), codes.pop().unwrap()),
                Some(code) => unimplemented!("code {:x}", code),
                None => panic!(),
            };
            instructions.push(inst);
        }
        instructions
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Self::Iconst3 => write!(f, "Iconst3")?,
            Self::Iconst5 => write!(f, "Iconst5")?,
            Self::Ldc(arg) => write!(f, "Ldc({})", arg)?,
            Self::Iload1 => write!(f, "Iload1")?,
            Self::Iload2 => write!(f, "Iload2")?,
            Self::Aload0 => write!(f, "Aload0")?,
            Self::Istore1 => write!(f, "Istore1")?,
            Self::Istore2 => write!(f, "Istore2")?,
            Self::Iadd => write!(f, "Iadd")?,
            Self::Return => write!(f, "Return")?,
            Self::GetStatic(arg1, arg2) => write!(f, "GetStatic({}, {})", arg1, arg2)?,
            Self::InvokeVirtual(arg1, arg2) => write!(f, "InvokeVirtual({}, {})", arg1, arg2)?,
            Self::Invokespecial(arg1, arg2) => write!(f, "Invokespecial({}, {})", arg1, arg2)?,
        };
        Ok(())
    }
}
