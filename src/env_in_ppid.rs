use super::Error;
use std::ffi::{OsStr, OsString};
use std::process::Command;

pub fn set<T, U>(pid: i32, var_name: T, var_value: U) -> Result<(), Error>
where
    T: AsRef<OsStr>,
    U: AsRef<OsStr>,
{
    let mut dbg_input = OsString::from(r#"call (size_t) setenv(""#);
    dbg_input.push(&var_name);
    dbg_input.push(r#"", ""#);
    dbg_input.push(&var_value);
    dbg_input.push(r#"")"#);

    let output = Command::new("gdb")
        .args(&[
            OsStr::new("-nx"),
            "-p".as_ref(),
            &pid.to_string().as_ref(),
            "--batch".as_ref(),
            "-ex".as_ref(),
            &dbg_input,
        ])
        .output()?;

    if !output.status.success() {
        Err(Error::ParentEnv(
            var_name.as_ref().into(),
            var_value.as_ref().into(),
            pid,
            output.stderr,
        ))?;
    }
    Ok(())
}
