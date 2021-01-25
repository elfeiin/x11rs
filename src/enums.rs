
#[derive(Debug)]
pub enum BitDepth {
   D1 = 1,
   D4 = 4,
   D8 = 8,
   D16 = 16,
   D24 = 24,
   D32 = 32,
}

#[derive(Debug)]
pub enum ScanLinePad {
   Pad8 = 8,
   Pad16 = 16,
   Pad32 = 32,
}

#[derive(Debug)]
pub enum UseBackingStores {
   Never,
   WhenMapped,
   Always,
}
pub enum BitGravity {
   Forget,
   NorthWest,
   North,
   NorthEast,
   West,
   Center,
   East,
   SouthWest,
   South,
   SouthEast,
   Static,
}
pub enum WinGravity {
   Unmap,
   NorthWest,
   North,
   NorthEast,
   West,
   Center,
   East,
   SouthWest,
   South,
   SouthEast,
   Static,
}
pub enum Event {
   KeyPress,
   KeyRelease,
   OwnerGrabButton,
   ButtonPress,
   ButtonRelease,
   EnterWindow,
   LeaveWindow,
   PointerMotion,
   PointerMotionHint,
   Button1Motion,
   Button2Motion,
   Button3Motion,
   Button4Motion,
   Button5Motion,
   ButtonMotion,
   Exposure,
   VisibilityChange,
   StructureNotify,
   ResizeRedirect,
   SubstructureNotify,
   SubstructureRedirect,
   FocusChange,
   PropertyChange,
   ColormapChange,
   KeymapState,
}
pub enum PointerEvent {
   ButtonPress,
   ButtonRelease,
   EnterWindow,
   LeaveWindow,
   PointerMotion,
   PointerMotionHint,
   Button1Motion,
   Button2Motion,
   Button3Motion,
   Button4Motion,
   Button5Motion,
   ButtonMotion,
   KeymapState,
}

pub enum DeviceEvent {
   KeyPress,
   KeyRelease,
   ButtonPress,
   ButtonRelease,
   PointerMotion,
   Button1Motion,
   Button2Motion,
   Button3Motion,
   Button4Motion,
   Button5Motion,
   ButtonMotion,
}
pub enum KeyMask {
   Shift,
   Lock,
   Control,
   Mod1,
   Mod2,
   Mod3,
   Mod4,
   Mod5,
}
pub enum ButMask {
   Button1,
   Button2,
   Button3,
   Button4,
   Button5,
}
pub enum KeyButMask {
   KeyMask(KeyMask),
   ButMask(ButMask),
}
pub enum Family {
   Internet,
   InternetV6,
   ServerInterpreted,
   DECnet,
   Chaos,
}
pub enum Error {
   Access,
   Alloc,
   Atom,
   Colormap,
   Cursor,
   Drawable,
   Font,
   GContext,
   IDChoice,
   Implementation,
   Length,
   Match,
   Name,
   Pixmap,
   Request,
   Value,
   Window,
}
pub enum Direction {
   RaiseLowest = 0,
   LowerHighest = 1,
}
pub enum Button {
   AnyButton = 0,
}
pub enum ChangePropertyMode {
   Replace = 0,
   Prepend = 1,
   Append = 2,
}