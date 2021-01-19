#[derive(Debug, PartialEq, Clone)]
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
