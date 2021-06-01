use super::*;

#[repr(C)]
#[derive(Debug)]
pub struct Format {
   pub depth: Card8,
   pub bits_per_pixel: Card8,
   pub scanline_pad: Card8,
}

impl From<[u8; 3]> for Format {
   fn from(v: [u8; 3]) -> Self {
      unsafe { std::mem::transmute::<[u8; 3], Format>(v) }
   }
}

impl Format {
   pub fn read_from(stream: &mut std::os::unix::net::UnixStream) -> Option<Self> {
      let mut format = [0u8; 3];
      
      match stream.read(&mut format) {
         Ok(_) => {
            match stream.read(&mut [0; 5]) {
               Ok(_) => (),
               Err(_) => return None,
            }
         },
         Err(_) => return None,
      }
      
      Some(format.into())
   }
}
