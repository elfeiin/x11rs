use super::*;

#[derive(Debug)]
pub struct Format {
   depth: Card8,
   bits_per_pixel: BitDepth,
   scanline_pad: ScanLinePad,
   // 5 byte unused
}