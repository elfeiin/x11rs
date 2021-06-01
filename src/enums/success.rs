#[derive(Debug)]
pub enum Success {
   Failed = 0,
   Success = 1,
   Authenticate = 2,
}

impl From<u8> for Success {
   fn from(b: u8) -> Self {
      match b {
         0 => Self::Failed,
         1 => Self::Success,
         2 => Self::Authenticate,
         _ => {
            println! {"got unknown response"};
            Self::Failed
         }
      }
   }
}

impl Default for Success {
   fn default() -> Self {
      Self::Failed
   }
}

impl std::fmt::Display for Success {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
      match self {
         Self::Failed => write! {f, "Failed"},
         Self::Success => write! {f, "Success"},
         Self::Authenticate => write!{f, "Authenticate"},
      }
   }
}
