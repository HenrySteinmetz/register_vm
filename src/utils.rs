pub trait ToU16 {
    fn to_u16(&self) -> u16;
}

impl ToU16 for [u8; 2] {
    fn to_u16(&self) -> u16 {
        ((self[0] as u16) << 8) | self[1] as u16
    }
}
