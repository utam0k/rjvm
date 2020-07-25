pub mod attribute;
pub mod constant_pool;
mod constant_pool_table;
pub mod method;

use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use constant_pool::ConstantPool;
use constant_pool_table::ConstantPoolTable;
use method::MethodInfo;

pub type ReaderResult<T> = Result<(T, Cursor<Vec<u8>>), Box<dyn std::error::Error>>;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
#[repr(C)]
pub struct Class {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    pub cp_info: ConstantPoolTable,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u8>,
    field_count: u16,
    field_info: Vec<u8>,
    method_count: u16,
    pub methods: Vec<MethodInfo>,
    attributes_count: u16,
    attribute_info: Vec<u8>,
}

impl Class {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let magic = rdr.read_u32::<BigEndian>()?;
        let minor_version = rdr.read_u16::<BigEndian>()?;
        let major_version = rdr.read_u16::<BigEndian>()?;
        let constant_pool_count = rdr.read_u16::<BigEndian>()?;
        let (cp_info, mut rdr) =
            (0..constant_pool_count - 1).try_fold((ConstantPoolTable::new(), rdr), |(mut cp_table, rdr), _i| {
                match ConstantPool::new(rdr) {
                    Ok((constant_pool, rdr2)) => {
                        cp_table.push(constant_pool);
                        Ok((cp_table, rdr2))
                    }
                    Err(err) => Err(err),
                }
            })?;

        let utf8_table = cp_info.utf8info();

        let access_flags = rdr.read_u16::<BigEndian>()?;
        let this_class = rdr.read_u16::<BigEndian>()?;
        let super_class = rdr.read_u16::<BigEndian>()?;

        let interfaces_count = rdr.read_u16::<BigEndian>()?;
        let interfaces = Vec::new(); // TODO: unimplemented!

        let field_count = rdr.read_u16::<BigEndian>()?;
        let field_info = Vec::new(); // TODO: unimplemented!

        let method_count = rdr.read_u16::<BigEndian>()?;
        let (methods, mut rdr) = (0..method_count).try_fold((Vec::new(), rdr), |(mut ret, rdr), _i| {
            match MethodInfo::new(rdr, &utf8_table) {
                Ok((method_info, rdr2)) => {
                    ret.push(method_info);
                    Ok((ret, rdr2))
                }
                Err(err) => Err(err),
            }
        })?;

        let attributes_count = rdr.read_u16::<BigEndian>()?;
        let attribute_info = Vec::new(); // TODO: unimplemented!

        Ok((
            Self {
                magic,
                minor_version,
                major_version,
                constant_pool_count,
                cp_info,
                access_flags,
                this_class,
                super_class,
                interfaces_count,
                interfaces,
                field_count,
                field_info,
                method_count,
                methods,
                attributes_count,
                attribute_info,
            },
            rdr,
        ))
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Class")?;
        writeln!(f, "\t magic: {:x}", self.magic)?;
        writeln!(f, "\t minor_version: {}", self.minor_version)?;
        writeln!(f, "\t major_version: {}", self.major_version)?;

        writeln!(f, "ConstantPool")?;
        for (i, constant_pool) in self.cp_info.iter().enumerate() {
            writeln!(f, "\t #{:2} {}", i + 1, constant_pool)?;
        }
        writeln!(f, "Interface")?;
        writeln!(f, "\t interface_count: {}", self.interfaces_count)?;
        writeln!(f, "Field")?;
        writeln!(f, "\t field_count: {}", self.field_count)?;
        writeln!(f, "Method")?;
        for method in &self.methods {
            writeln!(f, "{}", method)?;
        }
        writeln!(f, "Attribute")?;
        writeln!(f, "\t attributes_count: {}", self.field_count)?;
        Ok(())
    }
}
