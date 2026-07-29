//! Implements the DigOut channel.
//!
//! This should ultimately also be implemented automatically by the Macro.

/// Digital Output channels.
#[derive(Debug, Clone, Copy)]
pub enum DigOut {
    Out1,
    Out2,
    Out3,
    Out4,
    Out5,
    Out6,
    Out7,
    Out8,
    Out9,
    Out10,
    Out11,
    Out12,
    Out13,
    Out14,
    Out15,
    Out16,
}

impl DigOut {
    pub fn inner_to_writable(&self) -> String {
        match self {
            DigOut::Out1 => String::from("0"),
            DigOut::Out2 => String::from("1"),
            DigOut::Out3 => String::from("2"),
            DigOut::Out4 => String::from("3"),
            DigOut::Out5 => String::from("4"),
            DigOut::Out6 => String::from("5"),
            DigOut::Out7 => String::from("6"),
            DigOut::Out8 => String::from("7"),
            DigOut::Out9 => String::from("8"),
            DigOut::Out10 => String::from("9"),
            DigOut::Out11 => String::from("10"),
            DigOut::Out12 => String::from("11"),
            DigOut::Out13 => String::from("12"),
            DigOut::Out14 => String::from("13"),
            DigOut::Out15 => String::from("14"),
            DigOut::Out16 => String::from("15"),
        }
    }
}
