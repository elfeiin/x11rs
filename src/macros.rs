#[macro_export]
macro_rules! mpu {
   ($name:tt) => {
      mod $name;
      pub use $name::*;
   };
   () => {};
}

#[macro_export]
macro_rules! structure {
   ($name:ident, $($field:ident: $type:ty,)* ) => {
      pub struct $name {
         $(
            pub $field: $type,
         )*
      }
   };
   ($name:tt, $($field:tt: $type:ty),*) => {
      structure!{$name; $($field: $type)*}
   };
}


#[macro_export]
macro_rules! read_buf {
   ($stream:expr, $buf_len:expr) => {{
      let mut buf = [0; $buf_len];
      if match $stream.read(&mut buf) {
         Ok(n) => n,
         Err(_) => {
            return None;
         }
      } != $buf_len
      {
         return None;
      }
      buf
   }};
}

#[macro_export]
macro_rules! read_len {
   ($stream:expr, $len:expr) => {{
      let mut buf: Vec<u8> = vec![];
      let mut temp = [0u8];
      for _ in 0..$len {
         match $stream.read(&mut temp) {
            Ok(_) => (),
            Err(_) => {
               return None;
            }
         };
         buf.extend(&temp);
      }
      if buf.len() != $len {
         return None;
      }
      buf
   }};
}

#[macro_export]
macro_rules! read_u8 {
   ($stream:expr) => {
      read_buf!($stream, 1)[0]
   };
}

#[macro_export]
macro_rules! read_u16 {
   ($stream:expr) => {
      u16::from_le_bytes(read_buf!($stream, 2))
   };
}

#[macro_export]
macro_rules! read_u32 {
   ($stream:expr) => {
      u32::from_le_bytes(read_buf!($stream, 4))
   };
}

#[macro_export]
macro_rules! read_u64 {
   ($stream:expr) => {
      u64::from_le_bytes(read_buf!($stream, 8))
   };
}

// #[macro_export]
// #[macro_use]
// macro_rules! auto_enum {
//    ($name:tt -> $conv:tt, $($variant:tt = $v:expr),*) => {
//       pub enum $name {
//          $( $variant, )*
//       }
//       impl From<$name> for $conv {
//          fn from(e: $name) -> Self {
//             match e {
//                $( $name::$variant => $v, )*
//             }
//          }
//       }
//       impl From<$conv> for $name {
//          fn from(c: $conv) -> Self {
//             match c {
//                $( $v => $name::$variant, )*
//             }
//          }
//       }
//    };
//    ($name:tt -> $conv:tt, $($variant:tt = $v:expr,)*) => {
//       pub enum $name {
//          $( $variant, )*
//       }
//       impl From<$name> for $conv {
//          fn from(e: $name) -> Self {
//             match e {
//                $( $name::$variant => $v, )*
//             }
//          }
//       }
//       impl From<$conv> for $name {
//          fn from(c: $conv) -> Self {
//             match c {
//                $( $v => $name::$variant, )*
//             }
//          }
//       }
//    };
//    ($name:tt, $($variant:tt),*) => {
//       pub enum $name {
//          $( $variant, )*
//       }
//    };
//    ($name:tt, $($variant:tt,)*) => {
//       pub enum $name {
//          $( $variant, )*
//       }
//    };
// }
