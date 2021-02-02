#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum WindowClass {
   CopyfromParent = 0,
   InputOutput = 1,
   InputOnly = 2,
}

impl Default for WindowClass {
   fn default() -> Self {
      Self::CopyfromParent
   }
}

impl From<u8> for WindowClass {
   fn from(v: u8) -> Self {
      match v {
         0 => Self::CopyfromParent,
         1 => Self::InputOutput,
         _ => Self::InputOnly,
      }
   }
}
impl From<WindowClass> for u8 {
   fn from(v: WindowClass) -> Self {
      match v {
         WindowClass::CopyfromParent => 0,
         WindowClass::InputOutput => 1,
         WindowClass::InputOnly => 2,
      }
   }
}