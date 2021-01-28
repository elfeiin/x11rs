use super::*;

#[derive(Debug)]
pub struct Depth {
   pub depth: BitDepth,
   pub num_visual_types: Card16,
   pub visuals: Vec<VisualType>
}