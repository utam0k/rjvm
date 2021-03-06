use std::collections::HashMap;

use crate::class::constant_pool::ConstantPoolInfo;
use crate::class::method::MethodInfo;
use crate::class::Class;
use crate::instruction::Instruction;
use crate::operand_stack::{Item, OperandStack};

#[macro_export]
macro_rules! get_constant_pool {
    ($cp_pool: expr, $index: expr, $expect: tt) => {
        match &$cp_pool.get(($index - 1).into()).unwrap().info {
            $expect(val) => val,
            _v => unreachable!(),
        }
    };
}

type LocalVariable = HashMap<usize, Item>;

#[derive(Debug, Clone, Default)]
struct Frame {
    pub pc: usize,
    pub local_variable: LocalVariable,
    pub operand_stack: OperandStack,
}

impl Frame {
    pub fn new(local_variable: LocalVariable) -> Self {
        Self {
            pc: 0,
            local_variable,
            operand_stack: OperandStack::new(),
        }
    }
}

pub struct VM {
    class_info: Class,
    frames: Vec<Frame>,
}

impl VM {
    pub fn new(class_info: Class) -> Self {
        VM {
            class_info,
            frames: vec![],
        }
    }

    fn get_current_mut_frame(&mut self) -> &mut Frame {
        self.frames.last_mut().expect("A frame should be stacked.")
    }

    pub fn exec(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut local_variable = LocalVariable::new();
        local_variable.insert(0, Item::Classref(self.class_info.super_class as usize));
        let frame = Frame::new(local_variable);
        self.frames.push(frame);
        let main_method = self
            .class_info
            .methods
            .iter()
            .find(|&method| method.name == "main")
            .expect("The main method was not found.")
            .clone();

        self.exec_method(&main_method);

        Ok(())
    }

    fn exec_method(&mut self, method: &MethodInfo) {
        for code_attr in method.code_attribute() {
            for inst in &code_attr.instructions {
                if let Err(msg) = self.exec_per_inst(inst) {
                    panic!(msg)
                }
            }
        }
    }

    fn exec_per_inst(&mut self, inst: &Instruction) -> Result<(), String> {
        use ConstantPoolInfo::*;
        match inst {
            Instruction::Iconst3 => {
                let mut frame = self.get_current_mut_frame();
                frame.operand_stack.push(Item::Int(3));
                frame.pc += 1;
            }
            Instruction::Iconst5 => {
                let mut frame = self.get_current_mut_frame();
                frame.operand_stack.push(Item::Int(5));
                frame.pc += 1;
            }
            Instruction::Iload1 => {
                let mut frame = self.get_current_mut_frame();
                if let Some(val) = frame.local_variable.get(&1) {
                    frame.operand_stack.push(*val);
                } else {
                    return Err("Variable is not set to avalue".into());
                }
                frame.pc += 1;
            }
            Instruction::Iload2 => {
                let mut frame = self.get_current_mut_frame();
                if let Some(val) = frame.local_variable.get(&2) {
                    frame.operand_stack.push(*val);
                } else {
                    return Err("Variable is not set to avalue".into());
                }
                frame.pc += 1;
            }
            Instruction::Aload0 => {
                let mut frame = self.get_current_mut_frame();
                let val = frame.local_variable.get(&0).unwrap();
                frame.operand_stack.push(*val);
                frame.pc += 1;
            }
            Instruction::Istore1 => {
                let mut frame = self.get_current_mut_frame();
                let val = frame.operand_stack.pop().unwrap();
                frame.local_variable.insert(1, val);
                frame.pc += 1;
            }
            Instruction::Istore2 => {
                let mut frame = self.get_current_mut_frame();
                let val = frame.operand_stack.pop().unwrap();
                frame.local_variable.insert(2, val);
                frame.pc += 1;
            }
            Instruction::Iadd => {
                let frame = self.get_current_mut_frame();
                if let (Some(Item::Int(v2)), Some(Item::Int(v1))) =
                    (frame.operand_stack.pop(), frame.operand_stack.pop())
                {
                    frame.operand_stack.push(Item::Int(v1 + v2))
                } else {
                    panic!("Type Error");
                }
            }
            Instruction::Isub => {
                let frame = self.get_current_mut_frame();
                if let (Some(Item::Int(v2)), Some(Item::Int(v1))) =
                    (frame.operand_stack.pop(), frame.operand_stack.pop())
                {
                    frame.operand_stack.push(Item::Int(v1 - v2))
                } else {
                    panic!("Type Error");
                }
            }
            Instruction::Invokespecial(_, _) => self.get_current_mut_frame().pc += 3,
            Instruction::InvokeVirtual(_, method_index) => {
                let method_ref = get_constant_pool!(self.class_info.cp_info, method_index, MethodrefInfo);
                let name_and_type =
                    get_constant_pool!(self.class_info.cp_info, method_ref.name_and_type_index, NameAndTypeInfo);
                let method_name = get_constant_pool!(self.class_info.cp_info, name_and_type.name_index, Utf8Info);

                match &*method_name.to_string() {
                    "println" => {
                        let utf8info = self.class_info.cp_info.utf8info();
                        let mut frame = self.get_current_mut_frame();
                        match frame.operand_stack.pop() {
                            Some(Item::Int(index)) => println!("{}", index),
                            Some(Item::String(index)) => println!("{}", utf8info.get(&(index as u16)).unwrap()),
                            None => panic!(),
                            _ => unimplemented!(),
                        }
                        frame.pc += 3;
                    }
                    _ => unimplemented!(),
                }
            }
            Instruction::Ldc(index) => {
                // TODO: remove unwrap()
                let constant_pool = self.class_info.cp_info.get((index - 1).into()).unwrap();
                let val = match constant_pool.info {
                    ConstantPoolInfo::StringInfo(string_info) => string_info.bytes,
                    _ => unimplemented!(),
                };
                let mut frame = self.get_current_mut_frame();
                frame.operand_stack.push(Item::String(val.into()));
                frame.pc += 2;
            }
            Instruction::Return => self.get_current_mut_frame().pc += 1,
            Instruction::GetStatic(_, _) => {
                // TODO: unimplemented!
                // let index1 = code_attr.code.get(frame.pc + 1).unwrap();
                // let index2 = code_attr.code.get(frame.pc + 2).unwrap();
                // dbg!(index1, index2);
                // let symbol1 = self.class_info.cp_info.get(*index1 as usize + 1).unwrap();
                // let symbol2 = self.class_info.cp_info.get(*index2 as usize + 1).unwrap();
                // println!("{}", symbol1);
                // println!("{}", symbol2);
                let mut frame = self.get_current_mut_frame();
                frame.pc += 3;
                frame.operand_stack.push(Item::Int(0));
            }
        };
        Ok(())
    }
}
