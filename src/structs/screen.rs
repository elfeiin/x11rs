use super::*;

#[derive(Debug)]
pub struct Screen {
   pub root: Window,
   pub default_colormap: Colormap,
   pub white_pixel: Card32,
   pub black_pixel: Card32,
   pub current_input_masks: Card32, // What is SetofEVENT?
   pub width_in_pixels: Card16,
   pub height_in_pixels: Card16,
   pub width_in_millimeters: Card16,
   pub height_in_millimeters: Card16,
   pub min_installed_maps: Card16,
   pub max_installed_maps: Card16,
   pub root_visual: VisualID,
   pub backing_stores: UseBackingStores,
   pub save_unders: Bool,
   pub root_depth: Card8,
   pub num_allowed_depths: Card8,
   pub allowed_depths: Vec<Depth>,
}

impl Screen {
   pub fn new() -> Self {
      Self {
         root: 0,
         default_colormap: 0,
         white_pixel: 0,
         black_pixel: 0,
         current_input_masks: 0,
         width_in_pixels: 0,
         height_in_pixels: 0,
         width_in_millimeters: 0,
         height_in_millimeters: 0,
         min_installed_maps: 0,
         max_installed_maps: 0,
         root_visual: 0,
         backing_stores: UseBackingStores::Never,
         save_unders: false,
         root_depth: 0,
         num_allowed_depths: 0,
         allowed_depths: vec![],
      }
   }
}


impl Screen {
   pub fn read_from(stream: &mut std::os::unix::net::UnixStream) -> Option<Self> {
      
      let mut screen = Self::new();
      screen.root = read_u32! {stream};
      screen.default_colormap = read_u32! {stream};
      screen.white_pixel = read_u32! {stream};
      screen.black_pixel = read_u32! {stream};
      screen.current_input_masks = read_u32! {stream};
      screen.width_in_pixels = read_u16! {stream};
      screen.height_in_pixels = read_u16! {stream};
      screen.width_in_millimeters = read_u16! {stream};
      screen.height_in_millimeters = read_u16! {stream};
      screen.min_installed_maps = read_u16! {stream};
      screen.max_installed_maps = read_u16! {stream};
      screen.root_visual = read_u32! {stream};
      screen.backing_stores = read_u8! {stream}.into();
      screen.save_unders = match read_u8! {stream} {
         0 => false,
         _ => true,
      };
      screen.root_depth = read_u8! {stream};
      screen.num_allowed_depths = read_u8! {stream};
      
      for _ in 0..screen.num_allowed_depths {
         match Depth::read_from(stream) {
            Some(depth) => {
               screen.allowed_depths.push(depth);
            }
            None => (),
         }
      }
      
      Some(screen)
   }
}
