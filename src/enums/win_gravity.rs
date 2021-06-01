pub enum WinGravity {
   Unmap = 0,
   NorthWest = 1,
   North = 2,
   NorthEast = 3,
   West = 4,
   Center = 5,
   East = 6,
   SouthWest = 7,
   South = 8,
   SouthEast = 9,
   Static = 10,
}


impl Default for WinGravity {
   fn default() -> Self {
      Self::NorthWest
   }
}