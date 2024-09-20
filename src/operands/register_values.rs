use std::fmt::{Debug, Display};

#[derive(Copy, Clone)]
pub union RegisterValue {
    pub int: i64,
    pub float: f64,
    pub string_ptr: *const String,
    pub bool: bool,
}

impl PartialEq for RegisterValue {
    fn eq(&self, other: &Self) -> bool {
        let s_bytes = unsafe { self.int };
        let o_bytes = unsafe { other.int };
        s_bytes == o_bytes
    }
}

impl Display for RegisterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {},  {:?}",
            unsafe { self.int },
            unsafe { self.float },
            unsafe { self.bool },
            unsafe { self.string_ptr }
        )
    }
}

impl Default for RegisterValue {
    fn default() -> Self {
        Self { int: 0 }
    }
}

impl Debug for RegisterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RegisterValue with the following possible values: \n\tint: `{}`\n\tfloat: `{}`\n\tboolean: `{}`\n\tstring_pointer: `{:?}`\n\t",
            unsafe { self.int },
            unsafe { self.float },
            unsafe { self.bool },
            unsafe { self.string_ptr }
        )
    }
}
