# kx
Changes `KUBECONFIG` environment variable in the current shell

![](docs/demo.gif)

## Installation
You can either download compiled binary from [releases](releases) to a directory in you `$PATH` or run `cargo install kx` and compile it yourself.
Binaries in releases are build using `musl` toolchain, therefore they have no external dependencies (not even libc).

## Prerequisites
 - gdb

## Usage
```
$ kx
```

if you get this error:
```
Could not attach to process.  If your uid matches the uid of the target
process, check the setting of /proc/sys/kernel/yama/ptrace_scope, or try
again as the root user.  For more details, see /etc/sysctl.d/10-ptrace.conf
ptrace: Operation not permitted.
```
you might need:
```bash
# until restart:
sudo sysctl -w kernel.yama.ptrace_scope=0
# permanently:
sudo sed -i 's/kernel.yama.ptrace_scope = 1/kernel.yama.ptrace_scope = 0/' /etc/sysctl.d/10-ptrace.conf
```
more info: https://linux-audit.com/protect-ptrace-processes-kernel-yama-ptrace_scope/

## Configuration

 - `$KX_SEARCH_DIR` will let you choose KUBECONFIG from this directory
 - `$KX_CONFIG_PATH` will read configuration from this file (format below)
 - `$XDG_CONFIG_HOME/kx/config.json`
 - `~/.config/kx/config.json`
 - if none of them are found, will search for kubeconfigs in `~/.kube`

### Config file format
```json
{
    "search_dir": "/path/to/dir/with/cubeconfigs"
}
```
