#[derive(Debug)]
pub enum BitDepth {
   D1 = 1,
   D4 = 4,
   D8 = 8,
   D16 = 16,
   D24 = 24,
   D32 = 32,
}

impl From<u8> for BitDepth {
   fn from(v: u8) -> Self {
      match v {
         1 => Self::D1,
         4 => Self::D4,
         8 => Self::D8,
         16 => Self::D16,
         24 => Self::D24,
         32 => Self::D32,
         _ => Self::D32,
      }
   }
}
