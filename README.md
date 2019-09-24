# kx
Changes `KUBECONFIG` environment variable in the current shell

![](docs/demo.gif)

## Installation
You can either download compiled binary from [releases](releases) to a directory in you `$PATH` or compile it yourself by `cargo install k8s-kx`.
Binaries in releases are build using `musl` toolchain, therefore they have no external dependencies (not even libc).

Add the following to your `.bashrc`
```
alias kx='source <("k8s-kx")'
```
# Usage
```
$ kx
```

## Configuration
 - `$KX_SEARCH_DIR` will let you choose KUBECONFIG from this directory
 - `$KX_CONFIG_PATH` will read configuration from this file (format below)
 - `$XDG_CONFIG_HOME/kx/config.json`
 - `~/.config/kx/config.json`
 - if none of them are found, will search for kubeconfigs in `~/.kube`

### Config file format
```js
{
    "search_dir": [
        // absolute path
        "/path/to/dir/with/cubeconfigs",
        // will be resolved relative to $PWD
        "path/to/cubeconfigs", 
        // will be resolved relative to this config file
        "./path/to/cubeconfigs",
        // will be resolved relative to home directory
        "~/path/to/cubeconfigs",
        // will try to find first existing directory:
        //   $PWD/path/to/cubeconfigs
        //   $PWD/../path/to/cubeconfigs
        //   $PWD/../../path/to/cubeconfigs
        //   ...
        //   /path/to/cubeconfigs
        "^path/to/cubeconfigs",
    ]
}
```
