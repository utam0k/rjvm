use std::collections::HashMap;
use std::slice::Iter;

use crate::class::constant_pool::utf8_info::Utf8Info;
use crate::class::constant_pool::{ConstantPool, ConstantPoolInfo};

pub type Utf8Table = HashMap<u16, Utf8Info>;

pub struct ConstantPoolTable {
    pub table: Vec<ConstantPool>,
}

impl ConstantPoolTable {
    pub fn new() -> Self {
        ConstantPoolTable { table: Vec::new() }
    }

    pub fn iter(&self) -> Iter<ConstantPool> {
        self.table.iter()
    }

    pub fn push(&mut self, item: ConstantPool) {
        self.table.push(item)
    }

    pub fn utf8info(&self) -> Utf8Table {
        self.iter()
            .enumerate()
            .flat_map(|(i, cp)| match &cp.info {
                ConstantPoolInfo::Utf8Info(utf8info) => Some((i + 1, utf8info)),
                _ => None,
            })
            .fold(Utf8Table::new(), |mut hash, (index, utf8info)| {
                hash.insert(index as u16, utf8info.clone());
                hash
            })
    }
}

impl IntoIterator for ConstantPoolTable {
    type Item = ConstantPool;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.table.into_iter()
    }
}
