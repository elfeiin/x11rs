use super::*;

#[derive(Debug)]
pub struct Screen {
   root: Window,
   default_colormap: u8, // wrong colormap
   white_pixel: Card32,
   black_pixel: Card32,
   current_input_masks: u8, // Wrong set of event
   width_in_pixels: Card16,
   height_in_pixels: Card16,
   width_in_millimeters: Card16,
   height_in_mil_limeters: Card16,
   min_installed_maps: Card16,
   max_in_stalled_maps: Card16,
   root_visual: VisualID,
   backing_stores: UseBackingStores,
   save_unders: Bool,
   root_depth: Card8,
   num_allowed_depths: Card8,
   allowed_depths: Vec<BitDepth>, // wrong what is depth
}