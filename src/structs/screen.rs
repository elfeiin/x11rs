use super::*;

#[derive(Debug)]
pub struct Screen {
   pub root: Window,
   pub default_colormap: Colormap,
   pub white_pixel: Card32,
   pub black_pixel: Card32,
   pub current_input_masks: u8, // Wrong, should be set of event
   pub width_in_pixels: Card16,
   pub height_in_pixels: Card16,
   pub width_in_millimeters: Card16,
   pub height_in_mil_limeters: Card16,
   pub min_installed_maps: Card16,
   pub max_in_stalled_maps: Card16,
   pub root_visual: VisualID,
   pub backing_stores: UseBackingStores,
   pub save_unders: Bool,
   pub root_depth: Card8,
   pub num_allowed_depths: Card8,
   pub allowed_depths: Vec<BitDepth>,
}
