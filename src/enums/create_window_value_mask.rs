#[repr(u32)]
#[derive(Copy, Clone)]
pub enum CreateWindowValueMask {
   BackgroundPixmap = 0x1,
   BackgroundPixel = 0x2,
   BorderPixmap = 0x4,
   BorderPixel = 0x8,
   BitGravity = 0x10,
   WinGravity = 0x20,
   BackingStore = 0x40,
   BackingPlanes = 0x80,
   BackingPixel = 0x100,
   OverrideRedirect = 0x200,
   SaveUnder = 0x400,
   EventMask = 0x800,
   DoNotPropagateMask = 0x1000,
   Colormap = 0x2000,
   Cursor = 0x4000,
}
