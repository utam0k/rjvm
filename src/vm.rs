use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::class::constant_pool::ConstantPoolInfo;
use crate::class::Class;

#[derive(Debug, FromPrimitive, PartialEq)]
enum Instruction {
    Aload0 = 0x2a,
    InvokeVirtual = 0xb6,
    Invokespecial = 0xb7,
    Return = 0xb1,
    Ldc = 0x12,
    GetStatic = 0xb2,
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
        for method in &self.class_info.methods {
            let mut pc = 0;
            let mut sp = 0;
            for code_attr in &method.code_attribute() {
                loop {
                    match code_attr.code.get(pc) {
                        None => break,
                        Some(c) => {
                            if let Some(inst) = FromPrimitive::from_u8(*c) {
                                match inst {
                                    Instruction::Aload0 => {
                                        self.stack[self.bp + sp] = self.stack
                                            [self.bp + *c as usize - Instruction::Aload0 as usize];
                                        sp += 1;
                                        pc += 1;
                                    }
                                    Instruction::Invokespecial => pc += 3, // TODO: unimplemented!
                                    Instruction::InvokeVirtual => {
                                        let index = self.stack[self.bp + sp - 1];
                                        let arg_string = self
                                            .class_info
                                            .cp_info
                                            .utf8info()
                                            .get(&(index as u16))
                                            .unwrap()
                                            .clone();
                                        println!("{}", arg_string.to_string());
                                        pc += 3;
                                    }
                                    Instruction::Ldc => {
                                        // TODO: remove unwrap()
                                        pc += 1;
                                        let index = code_attr.code.get(pc).unwrap();
                                        let constant_pool = self
                                            .class_info
                                            .cp_info
                                            .get(*index as usize - 1)
                                            .unwrap();
                                        let val = match constant_pool.info {
                                            ConstantPoolInfo::StringInfo(string_info) => {
                                                string_info.bytes
                                            }
                                            _ => unimplemented!(),
                                        };
                                        self.stack[self.bp + sp] = val.into();
                                        sp += 1;
                                        pc += 1;
                                    }
                                    Instruction::Return => break,
                                    Instruction::GetStatic => {
                                        // TODO: unimplemented!
                                        sp += 1;
                                        pc += 3;
                                    }
                                };
                            } else {
                                unimplemented!()
                            };
                        }
                    }
                }
                self.bp += 512;
            }
        }
        Ok(())
    }
}
