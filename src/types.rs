use super::*;

pub type XID = u32;
pub type Window = XID;
pub type Pixmap = XID;
pub type Cursor = XID;
pub type Font = XID;
pub type GContext = XID;
pub type Colormap = XID;
pub type Drawable = XID;
pub type Fontable = XID;
pub type Atom = XID;
pub type VisualID = u32;
pub type Value = u32;
pub type Byte = u8;
pub type Int8 = i8;
pub type Int16 = i16;
pub type Int32 = i32;
pub type Card8 = u8;
pub type Card16 = u16;
pub type Card32 = u32;
pub type TimeStamp = Card32;
pub type Bool = bool;
pub type KeySym = XID;
pub type KeyCode = Card8;
pub type Button = Card8;
pub type String8 = Vec<Card8>;
pub type String16 = Vec<Char2B>;
