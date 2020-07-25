use std::fmt;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

use crate::class::ReaderResult;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.4.7
#[repr(C)]
#[derive(Clone)]
pub struct Utf8Info {
    length: u16,
    pub bytes: Vec<u8>,
}

impl Utf8Info {
    pub fn new(mut rdr: Cursor<Vec<u8>>) -> ReaderResult<Self> {
        let length = rdr.read_u16::<BigEndian>()?;
        let (bytes, rdr) = (0..length).try_fold((Vec::new(), rdr), |(mut ret, mut rdr), _i| match rdr.read_u8() {
            Ok(ch) => {
                ret.push(ch);
                Ok((ret, rdr))
            }
            Err(err) => Err(err),
        })?;
        Ok((Self { length, bytes }, rdr))
    }
}

impl fmt::Display for Utf8Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = std::str::from_utf8(&self.bytes).unwrap();
        write!(f, "{}", string)?;
        Ok(())
    }
}

impl fmt::Debug for Utf8Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = std::str::from_utf8(&self.bytes).unwrap();
        write!(f, "{}", string)?;
        Ok(())
    }
}
