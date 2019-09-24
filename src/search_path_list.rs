use std::iter::FromIterator;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SearchPathList(Vec<PathBuf>);

impl FromIterator<PathBuf> for SearchPathList {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = PathBuf>,
    {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for SearchPathList {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<PathBuf>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a SearchPathList {
    type Item = &'a PathBuf;
    type IntoIter = std::slice::Iter<'a, PathBuf>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
