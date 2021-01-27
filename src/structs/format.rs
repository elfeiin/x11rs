use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct Format {
   depth: Card8,
   bits_per_pixel: Card8,
   scanline_pad: Card8,
   unused: [u8; 5],
}

impl Format {
   pub fn several_from(iter: &mut dyn Iterator<Item = &u8>, num: usize) -> Vec<Self> {
      let mut result = iter.take(8 * num as usize).fold(
         ([0u8; 8], 0usize, vec![]),
         |(mut buf, mut read, mut fmts), i| {
            if read == 8 {
               read = 0;
               fmts.push(unsafe { std::mem::transmute::<[u8; 8], Format>(buf) });
            }
            buf[read] = *i;
            read += 1;

            (buf, read, fmts)
         },
      );
      if result.1 == 8 {
         unsafe {
            result
               .2
               .push(std::mem::transmute::<[u8; 8], Format>(result.0));
         }
      }
      result.2
   }
}
