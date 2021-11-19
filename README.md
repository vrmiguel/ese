# ese (Environment-Setting Exec)

`ese` is a lightweight dotenv-reading command runner

## Usage

```
Usage: ese [<command...>] [-f <file>] [-c] [-p]

A very lightweight environment variable manager

Options:
  -f, --file        the dotenv file to be loaded. Defaults to `ese.env` if not
                    set.
  -c, --clear       if set, the child process receives no env. variables except
                    those contained in the .env file.
  -p, --path-only   if set, the child process receives no env. variables except
                    the parent's PATH and the values contained in the .env file.
  --help            display usage information
```

The `-c/--clear` option makes sure that the child process does not receive any argument from the parent process.

The `-p/--path-only` option allows the child process to access the PATH env. variable of the parent process. This option ensures that parent's process doesn't get overriden by a PATH variable that may be located in the dotenv file.

