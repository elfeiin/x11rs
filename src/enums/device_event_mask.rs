#[repr(u32)]
#[derive(Copy, Clone)]
pub enum DeviceEventMask {
   None = 0,
   KeyPress = 0x00000001,
   KeyRelease = 0x00000002,
   ButtonPress = 0x00000004,
   ButtonRelease = 0x00000008,
   PointerMotion = 0x00000040,
   Button1Motion = 0x00000100,
   Button2Motion = 0x00000200,
   Button3Motion = 0x00000400,
   Button4Motion = 0x00000800,
   Button5Motion = 0x00001000,
   ButtonMotion = 0x00002000,
   UnusedMask = 0xFFFFC0B0,
}

impl Default for DeviceEventMask {
   fn default() -> Self {
      Self::None
   }
}