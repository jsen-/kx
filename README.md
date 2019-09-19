# kx
Changes KUBECONFIG environment variables in current shell

## prerequisites
 - gdb

## usage:
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
