
#[repr(u8)]
#[derive(Debug)]
pub enum UseBackingStores {
   Never = 0,
   WhenMapped = 1,
   Always = 2,
}

impl From<u8> for UseBackingStores {
   fn from(v: u8) -> Self {
      match v {
         1 => Self::WhenMapped,
         2 => Self::Always,
         _ => Self::Never,
      }
   }
}

impl Default for UseBackingStores {
   fn default() -> Self {
      0.into()
   }
}