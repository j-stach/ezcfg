
#[derive(Debug)]
pub enum Error {
    /// Error reading or writing file.
    Io(std::io::Error),
    /// Line is incorrectly formatted.
    Format(String),
    /// Failed to parse type from string.
    /// (This is kept generic to allow flexibility of types in the macro.)
    Parse(String, String),
    /// File is missing an expected field.
    Missing(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::Format(line) => write!(f, "Incorrect formatting: {}", line),
            Error::Parse(typ, string) => {
                write!(f, "Failed to parse {} from '{}'", typ, string)
            },
            Error::Missing(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err), 
            Error::Format(..) => None, 
            Error::Parse(..) => None, 
            Error::Missing(..) => None, 
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

