use std::fmt;

#[derive(Debug)]
pub enum RivotCliError {
    InvalidCmdError(String),
    InvalidArgError(String),
}

impl fmt::Display for RivotCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RivotCliError::InvalidArgError(msg) => write!(f, "{}", msg),
            RivotCliError::InvalidCmdError(cmd) => write!(f, "Invalid command: {}", cmd),
        }
    }
}
