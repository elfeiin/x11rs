use super::*;

#[derive(Debug)]
pub struct Depth {
   pub depth: BitDepth,
   pub num_visuals: Card16,
   pub visuals: Vec<VisualType>
}

impl Depth {
   pub fn new() -> Self {
      Self {
         depth: BitDepth::D1,
         num_visuals: 0,
         visuals: vec![],
      }
   }
}

impl Depth {
   pub fn read_from(stream: &mut std::os::unix::net::UnixStream) -> Option<Self> {
      let mut depth = Depth::new();
      
      depth.depth = read_u8! {stream}.into();
      read_u8! {stream};
      depth.num_visuals = read_u16! {stream};
      read_u32! {stream};
      
      for _ in 0..depth.num_visuals {
         match VisualType::read_from(stream) {
            Some(visual_type) => {
               depth.visuals.push(visual_type);
            }
            None => (),
         }
      }
      
      Some(depth)
   }
}