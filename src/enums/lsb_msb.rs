#[derive(Copy, Clone, Debug)]
pub enum BitOrder {
   MSBFirst = 0o102,
   LSBFirst = 0o154,
}
impl From<BitOrder> for u8 {
   fn from(bo: BitOrder) -> Self {
      match bo {
         BitOrder::LSBFirst => 0,
         _ => 1,
      }
   }
}
impl From<u8> for BitOrder {
   fn from(b: u8) -> Self {
      match b {
         0 => Self::LSBFirst,
         _ => Self::MSBFirst,
      }
   }
}

#[derive(Copy, Clone, Debug)]
pub enum ByteOrder {
   MSBFirst = 0o102,
   LSBFirst = 0o154,
}
impl From<ByteOrder> for u8 {
   fn from(bo: ByteOrder) -> Self {
      match bo {
         ByteOrder::MSBFirst => 0o102,
         _ => 0o154,
      }
   }
}
impl From<u8> for ByteOrder {
   fn from(b: u8) -> Self {
      match b {
         0o102 => Self::MSBFirst,
         _ => Self::LSBFirst,
      }
   }
}
impl Default for ByteOrder {
   fn default() -> Self {
      Self::MSBFirst
   }
}