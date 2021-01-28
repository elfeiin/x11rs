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
   pub unused: u32,
}