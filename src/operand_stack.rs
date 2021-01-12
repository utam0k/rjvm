// pub enum Item {
//     Int(i32),
//     Classref(usize),
//     Fieldref(usize),
//     Objectref(usize),
//     Arrayref(usize),
// }
type Item = u64;

#[derive(Debug, Clone, Default)]
pub struct OperandStack {
    pub stack: Vec<Item>,
}

impl OperandStack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push(&mut self, item: Item) -> () {
        self.stack.push(item)
    }

    pub fn pop(&mut self) -> Option<Item> {
        self.stack.pop()
    }
}
