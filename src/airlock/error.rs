use std::fmt;

#[derive(Debug)]
pub enum Error {
    IllegalAction,
    UnchangedState,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IllegalAction => write!(f, "{}", ::std::error::Error::description(self)),
            Error::UnchangedState => write!(f, "{}", ::std::error::Error::description(self)),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IllegalAction => "Illegal action",
            Error::UnchangedState => "Unchanged state",
        }
    }
}
