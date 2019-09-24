use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    HomeDir,
    Io(io::Error),
    Config(PathBuf, io::Error),
    ConfigRelativeWithoutConfig,
    ConfigDeserialize(PathBuf, serde_json::Error),
    SearchDirNotFoundUp(PathBuf),
    SearchDirNotFound(PathBuf),
    SearchDirNotDir(PathBuf),
    SearchPathEmpty,
}

impl Error {
    pub fn print(self) -> Self {
        eprintln!("{}", self);
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HomeDir => write!(f, "Unable to determine your home directory"),
            Error::Io(e) => write!(f, "{}", e),
            Error::Config(path, ioerr) => write!(
                f,
                "Error reading config file \"{}\":\n{}",
                path.display(),
                ioerr,
            ),
            Error::ConfigDeserialize(path, json_error) => write!(
                f,
                "Error deserializing config \"{}\":\n{}",
                path.display(),
                json_error,
            ),
            Error::SearchPathEmpty => write!(f, "Search path is empty"),
            Error::ConfigRelativeWithoutConfig => write!(
                f,
                "Config relative path cannot be defined through environment variable"
            ),
            Error::SearchDirNotFoundUp(path) => {
                write!(f, "Searching up \"{}\" ... not found", path.display())
            }
            Error::SearchDirNotFound(path) => {
                write!(f, "Search dir \"{}\" does not exist", path.display())
            }
            Error::SearchDirNotDir(path) => {
                write!(f, "Search dir \"{}\" is not a directory", path.display())
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
