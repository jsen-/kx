use super::Error;
use std::ffi::{OsStr, OsString};
use std::process::Command;

pub fn set<T, U>(pid: i32, var_name: T, var_value: U) -> Result<(), Error>
where
    T: AsRef<OsStr>,
    U: AsRef<OsStr>,
{
    let var_name = var_name.as_ref();
    let var_value = var_value.as_ref();

    let mut dbg_input = OsString::from(r#"call (int) setenv(""#);
    dbg_input.push(&var_name);
    dbg_input.push(r#"", ""#);
    dbg_input.push(&var_value);
    dbg_input.push(r#"")"#);

    let output = Command::new("gdb")
        .args(&[
            OsStr::new("--nx"),
            "--readnever".as_ref(),
            "--pid".as_ref(),
            &pid.to_string().as_ref(),
            "--eval-command".as_ref(),
            &dbg_input,
            "--batch".as_ref(),
        ])
        .output()?;

    if !output.status.success() {
        Err(Error::ParentEnv(
            var_name.into(),
            var_value.into(),
            pid,
            output.stderr,
        ))?;
    }
    Ok(())
}
