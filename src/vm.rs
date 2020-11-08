use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::class::attribute::code::CodeAttribute;
use crate::class::constant_pool::ConstantPoolInfo;
use crate::class::method::MethodInfo;
use crate::class::Class;

#[derive(Debug, FromPrimitive, PartialEq)]
enum Instruction {
    Ldc = 0x12,
    Aload0 = 0x2a,
    Return = 0xb1,
    GetStatic = 0xb2,
    InvokeVirtual = 0xb6,
    Invokespecial = 0xb7,
}

struct Frame {
    pub pc: usize,
    pub sp: usize,
    pub method_info: MethodInfo,
}

impl Frame {
    pub fn new(method_info: MethodInfo) -> Self {
        Self {
            pc: 0,
            sp: 0,
            method_info,
        }
    }
}

pub struct VM {
    class_info: Class,
    bp: usize,
    stack: Vec<u64>,
}

impl VM {
    pub fn new(class_info: Class) -> Self {
        VM {
            class_info,
            bp: 0,
            stack: vec![0; 1024],
        }
    }

    pub fn exec(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut frames: Vec<Frame> = self.class_info.methods.iter().map(|m| Frame::new(m.clone())).collect();
        for mut frame in &mut frames {
            for code_attr in frame.method_info.code_attribute() {
                loop {
                    match code_attr.code.get(frame.pc) {
                        None => break,
                        Some(c) => {
                            if let Some(inst) = FromPrimitive::from_u8(*c) {
                                if let Some((pc, sp)) = self.exec_per_inst(inst, code_attr, frame) {
                                    frame.sp += sp;
                                    frame.pc += pc;
                                } else {
                                    break;
                                }
                            } else {
                                unimplemented!()
                            };
                        }
                    }
                }
                self.bp += frame.sp;
            }
        }
        Ok(())
    }

    fn exec_per_inst(&mut self, inst: Instruction, code_attr: &CodeAttribute, frame: &Frame) -> Option<(usize, usize)> {
        match inst {
            Instruction::Aload0 => {
                self.stack[self.bp + frame.sp] = self.stack[self.bp + 0];
                return Some((1, 1));
            }
            Instruction::Invokespecial => return Some((3, 0)),
            Instruction::InvokeVirtual => {
                let index = self.stack[self.bp + frame.sp - 1];
                let arg_string = self.class_info.cp_info.utf8info().get(&(index as u16)).unwrap().clone();
                println!("{}", arg_string.to_string());
                return Some((3, 0));
            }
            Instruction::Ldc => {
                // TODO: remove unwrap()
                let index = code_attr.code.get(frame.pc + 1).unwrap();
                let constant_pool = self.class_info.cp_info.get(*index as usize - 1).unwrap();
                let val = match constant_pool.info {
                    ConstantPoolInfo::StringInfo(string_info) => string_info.bytes,
                    _ => unimplemented!(),
                };
                self.stack[self.bp + frame.sp] = val.into();
                return Some((2, 1));
            }
            Instruction::Return => return None,
            Instruction::GetStatic => {
                // TODO: unimplemented!
                return Some((3, 1));
            }
        };
    }
}
