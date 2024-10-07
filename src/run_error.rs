#[derive(Debug)]
pub enum RunError {
    IoError(std::io::Error),
    ParseError(String),
}

impl From<std::io::Error> for RunError {
    fn from(error: std::io::Error) -> Self {
        RunError::IoError(error)
    }
}
