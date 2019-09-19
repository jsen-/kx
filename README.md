# kx
Changes KUBECONFIG environment variables in current shell

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
you might need to execute:
```bash
sudo sysctl -w kernel.yama.ptrace_scope=0
# more info: https://stackoverflow.com/questions/45171339/gdb-cannot-attach-to-process
```

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
