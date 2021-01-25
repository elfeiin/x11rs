use super::*;

#[derive(Debug)]
pub enum UseBackingStores {
   Never,
   WhenMapped,
   Always,
}