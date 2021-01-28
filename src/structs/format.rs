use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct Format {
   pub depth: Card8,
   pub bits_per_pixel: Card8,
   pub scanline_pad: Card8,
}

impl Format {
   pub fn several_from(v: Vec<u8>, num: usize) -> Vec<Self> {
      let mut output = vec![];
      let mut buf = [0; 3];
      let mut read = 0;
      for b in v {
         if read < 3 {
            buf[read] = b;
         }
         if read == 3 {
            output.push(unsafe { std::mem::transmute::<[u8; 3], Format>(buf) });
         }
         if read == 8 {
            buf = [0; 3];
            read = 0;
         }
         read += 1;
      }
      output
      // let mut result = iter.take(8 * num as usize).fold(
      //    ([0u8; 8], 0usize, vec![]),
      //    |(mut buf, mut read, mut fmts), i| {
      //       if read == 8 {
      //          read = 0;
      //          fmts.push(unsafe { std::mem::transmute::<[u8; 8], Format>(buf) });
      //       }
      //       buf[read] = *i;
      //       read += 1;

      //       (buf, read, fmts)
      //    },
      // );
      // if result.1 == 8 {
      //    unsafe {
      //       result
      //          .2
      //          .push(std::mem::transmute::<[u8; 8], Format>(result.0));
      //    }
      // }
      // result.2
   }
}
