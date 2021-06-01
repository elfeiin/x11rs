use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct VisualType {
   pub visual_id: Card32,
   pub visual_class: VisualClass,
   pub bits_per_rgb_value: Card8,
   pub colormap_entries: Card16,
   pub red_mask: Card32,
   pub green_mask: Card32,
   pub blue_mask: Card32,
   pub unused: Card32,
}

impl From<[u8; 24]> for VisualType {
   fn from(v: [u8; 24]) -> Self {
      unsafe { std::mem::transmute::<[u8; 24], VisualType>(v) }
   }
}

impl VisualType {
   pub fn read_from(stream: &mut std::os::unix::net::UnixStream) -> Option<Self> {
      let buf = read_buf! {stream, 24};
      
      Some(buf.into())
   }
}
