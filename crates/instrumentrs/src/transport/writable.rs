//! The writable trait.
//!
//! This trait controls what can be written to an interface and allows to define instruments and
//! transport generically.

pub trait Writable {
    fn to_byte_slice(&self) -> &[u8];
}

impl Writable for &str {
    fn to_byte_slice(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Writable for String {
    fn to_byte_slice(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Writable for Vec<u8> {
    fn to_byte_slice(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Writable for &[u8] {
    fn to_byte_slice(&self) -> &[u8] {
        self
    }
}
