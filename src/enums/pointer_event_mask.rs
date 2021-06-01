#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PointerEventMask {
   None = 0,
   ButtonPress = 0x00000004,
   ButtonRelease = 0x00000008,
   EnterWindow = 0x00000010,
   LeaveWindow = 0x00000020,
   PointerMotion = 0x00000040,
   PointerMotionHint = 0x00000080,
   Button1Motion = 0x00000100,
   Button2Motion = 0x00000200,
   Button3Motion = 0x00000400,
   Button4Motion = 0x00000800,
   Button5Motion = 0x00001000,
   ButtonMotion = 0x00002000,
   KeymapState = 0x00004000,
   UnusedMask = 0xFFFF8003,
}