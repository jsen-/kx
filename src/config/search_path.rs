use super::findup::findup;
use crate::Error;
use os_str_generic::OsStrGenericExt;
use std::env::current_dir;
use std::path::{Component, PathBuf};

#[derive(Debug, PartialEq)]
pub enum SearchPath {
    Absolute(PathBuf),
    ConfigRelative {
        config: Option<PathBuf>,
        search_path: PathBuf,
    },
    Relative(PathBuf),
    HomeRelative(PathBuf),
    SearchUp(PathBuf),
    Empty,
}

impl SearchPath {
    pub fn new(path: PathBuf, config: Option<PathBuf>) -> SearchPath {
        if let Some(first_component) = path.components().next() {
            match first_component {
                Component::Prefix(_) => SearchPath::Absolute(path),
                Component::RootDir => SearchPath::Absolute(path),
                Component::CurDir => SearchPath::ConfigRelative {
                    config,
                    search_path: path,
                },
                Component::ParentDir => SearchPath::Relative(path),
                Component::Normal(_) => match path.as_os_str().without_prefix("^") {
                    None => match path.as_os_str().without_prefix("~/") {
                        None => SearchPath::Relative(path),
                        Some(remainder) => SearchPath::HomeRelative(PathBuf::from(remainder)),
                    },
                    Some(remainder) => SearchPath::SearchUp(remainder.into()),
                },
            }
        } else {
            SearchPath::Empty
        }
    }

    fn should_exist(&self) -> bool {
        match self {
            SearchPath::Absolute { .. }
            | SearchPath::ConfigRelative { .. }
            | SearchPath::Relative { .. }
            | SearchPath::HomeRelative { .. } => true,
            SearchPath::SearchUp { .. } | SearchPath::Empty { .. } => false,
        }
    }

    pub fn resolve(&self) -> Result<PathBuf, Error> {
        let path = match self {
            SearchPath::Absolute(path) => path.clone(),
            SearchPath::Empty => Err(Error::SearchPathEmpty)?,
            SearchPath::ConfigRelative {
                config,
                search_path: path,
            } => match config {
                None => Err(Error::ConfigRelativeWithoutConfig)?,
                Some(config) => config.parent().expect("config file shall be in a directory").join(path),
            },
            SearchPath::Relative(path) => current_dir()?.join(path),
            SearchPath::HomeRelative(path) => dirs::home_dir().ok_or(Error::HomeDir)?.join(path),
            SearchPath::SearchUp(path) => match findup(path, current_dir()?) {
                None => Err(Error::SearchDirNotFoundUp(path.clone()))?,
                Some(path) => path,
            },
        };
        if self.should_exist() {
            if !path.exists() {
                return Err(Error::SearchDirNotFound(path));
            } else if !path.is_dir() {
                return Err(Error::SearchDirNotDir(path));
            }
        }
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_path(p: &str) -> SearchPath {
        SearchPath::new(PathBuf::from(p), None)
    }

    #[test]
    fn test1() {
        assert_eq!(new_path("/"), SearchPath::Absolute(PathBuf::from("/")));
        assert_eq!(new_path(""), SearchPath::Empty);
        assert_eq!(
            new_path("./"),
            SearchPath::ConfigRelative {
                search_path: PathBuf::from("./"),
                config: None
            }
        );
        assert_eq!(
            new_path("hello"),
            SearchPath::Relative(PathBuf::from("hello"))
        );
        assert_eq!(
            new_path("^hello"),
            SearchPath::SearchUp(PathBuf::from("hello"))
        );
    }
}
