use std::collections::HashMap;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::class::attribute::code::CodeAttribute;
use crate::class::constant_pool::ConstantPoolInfo;
use crate::class::Class;
use crate::operand_stack::OperandStack;

#[macro_export]
macro_rules! get_constant_pool {
    ($cp_pool: expr, $index: expr, $expect: tt) => {
        match &$cp_pool.get($index as usize - 1).unwrap().info {
            $expect(val) => val,
            _v => unreachable!(),
        }
    };
}

#[derive(Debug, FromPrimitive, PartialEq)]
enum Instruction {
    Iconst5 = 0x08,
    Ldc = 0x12,
    Iload1 = 0x1b,
    Aload0 = 0x2a,
    Istore1 = 0x3c,
    Return = 0xb1,
    GetStatic = 0xb2,
    InvokeVirtual = 0xb6,
    Invokespecial = 0xb7,
}

struct Frame {
    pub pc: usize,
    pub local_variable: HashMap<usize, u64>, // TODO: implement stack and unification value
    pub operand_stack: OperandStack,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            pc: 0,
            local_variable: HashMap::new(),
            operand_stack: OperandStack::new(),
        }
    }
}

pub struct VM {
    class_info: Class,
}

impl VM {
    pub fn new(class_info: Class) -> Self {
        VM { class_info }
    }

    pub fn exec(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: remove clone()
        for method in &self.class_info.methods.clone() {
            for code_attr in method.code_attribute() {
                let mut frame = Frame::new();
                loop {
                    match code_attr.code.get(frame.pc) {
                        None => break,
                        Some(c) => {
                            if let Some(inst) = FromPrimitive::from_u8(*c) {
                                if let Err(msg) = self.exec_per_inst(inst, &code_attr, &mut frame) {
                                    panic!(msg)
                                }
                            } else {
                                unimplemented!("code: {:0x}", c)
                            };
                        }
                    }
                    if code_attr.code_length == frame.pc as u32 {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn exec_per_inst(&mut self, inst: Instruction, code_attr: &CodeAttribute, frame: &mut Frame) -> Result<(), String> {
        use ConstantPoolInfo::*;
        match inst {
            Instruction::Iconst5 => {
                frame.operand_stack.push(5);
                frame.pc += 1;
            }
            Instruction::Iload1 => {
                if let Some(val) = frame.local_variable.get(&1) {
                    frame.operand_stack.push(*val);
                } else {
                    return Err("Variable is not set to avalue".into());
                }
                frame.pc += 1;
            }
            Instruction::Aload0 => {
                // frame.operand_stack.push(frame.operand_stack.pop().unwrap());
                frame.pc += 1;
            }
            Instruction::Istore1 => {
                let val = frame.operand_stack.pop().unwrap();
                frame.local_variable.insert(1, val);
                frame.pc += 1;
            }
            Instruction::Invokespecial => frame.pc += 3,
            Instruction::InvokeVirtual => {
                let index = frame.operand_stack.pop().unwrap();

                let method_index = code_attr.code.get(frame.pc + 2).unwrap();
                let method_ref = get_constant_pool!(self.class_info.cp_info, *method_index, MethodrefInfo);
                let name_and_type =
                    get_constant_pool!(self.class_info.cp_info, method_ref.name_and_type_index, NameAndTypeInfo);
                let method_name = get_constant_pool!(self.class_info.cp_info, name_and_type.name_index, Utf8Info);

                match &*method_name.to_string() {
                    "println" => {
                        // let arg_string = self.class_info.cp_info.utf8info().get(&(index as u16)).unwrap().clone();
                        if let Some(arg_string) = self.class_info.cp_info.utf8info().get(&(index as u16)) {
                            println!("{}", arg_string.to_string());
                        } else {
                            println!("{}", index);
                        }
                    }
                    _ => unimplemented!(),
                }

                frame.pc += 3;
            }
            Instruction::Ldc => {
                // TODO: remove unwrap()
                let index = code_attr.code.get(frame.pc + 1).unwrap();
                let constant_pool = self.class_info.cp_info.get(*index as usize - 1).unwrap();
                let val = match constant_pool.info {
                    ConstantPoolInfo::StringInfo(string_info) => string_info.bytes,
                    _ => unimplemented!(),
                };
                frame.operand_stack.push(val.into());
                frame.pc += 2;
            }
            Instruction::Return => frame.pc += 1,
            Instruction::GetStatic => {
                // TODO: unimplemented!
                // let index1 = code_attr.code.get(frame.pc + 1).unwrap();
                // let index2 = code_attr.code.get(frame.pc + 2).unwrap();
                // dbg!(index1, index2);
                // let symbol1 = self.class_info.cp_info.get(*index1 as usize + 1).unwrap();
                // let symbol2 = self.class_info.cp_info.get(*index2 as usize + 1).unwrap();
                // println!("{}", symbol1);
                // println!("{}", symbol2);
                frame.pc += 3;
                frame.operand_stack.push(0);
            }
        };
        Ok(())
    }
}
