// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum PkmnError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),
    #[error(transparent)]
    Var(#[from] std::env::VarError),
    #[error(transparent)]
    ConnectionError(#[from] diesel::ConnectionError),
    #[error(transparent)]
    StringError(#[from] StringError),
}

// we must manually implement serde::Serialize
impl serde::Serialize for PkmnError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type PkmnResult<T> = Result<T, PkmnError>;

// error1.rs
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct StringError {
    msg: String,
}

impl StringError {
    pub fn new(msg: &str) -> Self {
        StringError {
            msg: msg.to_string(),
        }
    }

    pub fn err(self) -> Result<(), Self> {
        Err(self)
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        &self.msg
    }
}
