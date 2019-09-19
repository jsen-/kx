#![allow(unused_variables, dead_code, unused_mut)]

mod config;
mod env_in_ppid;
// mod kubeconfig;

use itertools::Itertools;

use skim::{Skim, SkimOptionsBuilder};
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io;
use std::io::{copy, Cursor};
use std::os::unix::ffi::OsStringExt;
use std::process;
use sysinfo::{ProcessExt, SystemExt};

#[derive(Debug)]
pub enum Error {
    HomeDir,
    Io(io::Error),
    ParentEnv(OsString, OsString, i32, Vec<u8>),
    Config(io::Error),
    Pid,
    ParentPid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::HomeDir => write!(f, "Unable to determine your home directory"),
            Error::Io(e) => write!(f, "{}", e),
            Error::ParentEnv(var_name, var_value, pid, err_output) => write!(
                f,
                r#"Could not set environment variable "{:?}={:?}" in parent process pid {}:
{}"#,
                &var_name,
                &var_value,
                pid,
                String::from_utf8_lossy(err_output)
            ),
            Error::Config(ioerr) => write!(f, "Error reading config:\n{}", ioerr,),
            Error::Pid => write!(f, "Unable to determin process id"),
            Error::ParentPid => write!(f, "Unable to determin parent process id"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn real_main() -> Result<(), Error> {
    let home_dir = dirs::home_dir().ok_or_else(|| Error::HomeDir)?;
    let mut kcfgs = Vec::new();
    let srcdir = home_dir.join(".kube");
    for entry in fs::read_dir(&srcdir)? {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            kcfgs.push(entry.file_name());
        }
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let mut skim_input = Vec::new();

    for s in kcfgs.into_iter().intersperse(OsString::from("\n")) {
        let vec = s.into_vec();
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

    let path = srcdir.join(&item.get_output_text().into_owned());

    const VAR_NAME: &str = "KUBECONFIG";

    let ppid = sysinfo::System::new()
        .get_process(process::id() as i32)
        .ok_or(Error::Pid)?
        .parent()
        .ok_or(Error::ParentPid)?;

    env_in_ppid::set(ppid, VAR_NAME, &path)?;
    println!("{}={}", VAR_NAME, path.display());

    // let output = process::Command::new("kubectl")
    //     .args(&["config", "view", "-o", "json"])
    //     .output()?;

    // let kcfg = kubeconfig::read(&output.stdout)?;

    // let input = "11111\n22222\n333333333".to_string();

    // let selected_items = Skim::run_with(&options, Some(Box::new(Cursor::new(input))))
    //     .map(|out| out.selected_items)
    //     .unwrap_or_else(|| Vec::new());

    // for item in selected_items.iter() {
    //     print!("{}: {}{}", item.get_index(), item.get_output_text(), "\n");
    // }
    Ok(())
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("{}", e);
    }
}
