mod config;
mod error;
mod search_path_list;

pub use error::Error;
use search_path_list::SearchPathList;

use itertools::Itertools;
use skim::{Skim, SkimOptionsBuilder};
use std::fs;
use std::io::Write;
use std::io::{copy, Cursor};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

pub fn real_main() -> Result<(), Error> {
    let config = config::settings()?;
    let mut kcfgs = Vec::new();

    let spl: SearchPathList = config
        .search_paths
        .into_iter()
        .filter_map(|search_path| match search_path.resolve() {
            Ok(abs_path) => Some(abs_path),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        })
        .collect();

    for abs_path in &spl {
        for entry in fs::read_dir(&abs_path)? {
            let entry = entry?;
            if entry.metadata()?.is_file() {
                kcfgs.push(entry.path());
            }
        }
    }
    if kcfgs.is_empty() {
        let err = std::io::stderr();
        let mut stderr = err.lock();
        write!(&mut stderr, "No files found in search directories: ")?;
        for path in spl.into_iter().intersperse(PathBuf::from(", ")) {
            write!(&mut stderr, "{}", path.display())?;
        }
        writeln!(&mut stderr)?;
    } else {
        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .build()
            .unwrap();
        let mut skim_input = Vec::new();

        kcfgs.sort();
        for s in kcfgs.into_iter().intersperse(PathBuf::from("\n")) {
            let vec = s.into_os_string().into_vec();
            let mut v = Cursor::new(&vec);
            copy(&mut v, &mut skim_input)?;
        }

        let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(skim_input))))
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());
        let item = if let Some(selected) = selected_items.into_iter().next() {
            selected
        } else {
            return Ok(());
        };

        let path = PathBuf::from(item.get_output_text().into_owned());

        let out = std::io::stdout();
        let mut stdout = out.lock();
        write!(&mut stdout, "export KUBECONFIG=\"")?;
        stdout.write(path.as_os_str().as_bytes())?;
        write!(&mut stdout, "\"")?;
    }

    Ok(())
}

fn main() {
    real_main().unwrap();
    // if let Err(e) = real_main() {
    //     eprintln!("{}", e);
    // }
}
