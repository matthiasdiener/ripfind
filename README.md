# ripfind
A fast and lean replacement for the find command, modeled after the ripgrep package (https://github.com/BurntSushi/ripgrep).

## Usage:

```
rf [regex] [dir]
```

Prints all files matching `<regex>` in the directory `<dir>`.


### Options

`<dir>` is optional, if it is not provided ripfind will search in the current directory.
`<regex>` is optional,  if it is not provided ripfind will print all files.
