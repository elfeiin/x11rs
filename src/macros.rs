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
