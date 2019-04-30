use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    repr: Repr
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match &self.repr {
            Repr::Simple(e) => *e,
            Repr::Custom(c) => c.kind 
        }
    }
}

#[derive(Debug)]
pub enum Repr {
    Simple(ErrorKind),
    Custom(CustomError)
}

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    ExternalError,
    TestingError
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::ExternalError => "error originating externally to this crate",
            ErrorKind::TestingError => "function is implemented as a placeholder"
        }
    }
}

#[derive(Debug)]
pub struct CustomError {
    kind: ErrorKind,
    error: Box<dyn error::Error+Send+Sync>
}

impl From<CustomError> for Error {
    fn from(e: CustomError) -> Self { 
        Self { repr: Repr::Custom(e) } 
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self { 
        Self { repr: Repr::Simple(e) } 
    }
}

impl From<serde_json::Error> for Error {
    fn from(ser: serde_json::Error) -> Self {
        Self {
            repr: Repr::Custom(
                CustomError {
                    kind: ErrorKind::ExternalError,
                    error: Box::new(ser)
                }
            )
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) }
}

impl error::Error for Error {
    
    fn description(&self) -> &str { self.kind().as_str() }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.repr {
            Repr::Simple(_) => None,
            Repr::Custom(ref c) => c.error.source()
        }
    }
}