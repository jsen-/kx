use std::path::{Path, PathBuf};

fn findup_(file_name: &Path, relative_to: &Path) -> Option<PathBuf> {
    if file_name.is_absolute() {
        return if file_name.exists() {
            Some(file_name.to_path_buf())
        } else {
            None
        };
    }
    let mut curr = Some(relative_to);

    loop {
        if let Some(c) = curr {
            let abs = c.join(file_name);
            if abs.exists() {
                return Some(abs.to_path_buf());
            }
            curr = c.parent();
        } else {
            let abs = Path::new("/").join(file_name);
            if abs.exists() {
                return Some(abs.to_path_buf());
            } else {
                return None;
            }
        }
    }
}

pub fn findup<A, B>(file_name: A, relative_to: B) -> Option<PathBuf>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    findup_(file_name.as_ref(), relative_to.as_ref())
}
