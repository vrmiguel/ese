# ese (Environment-Setting Exec)

`ese` is a lightweight dotenv-reading command runner

## Usage

```
Usage: ese [<command...>] [-f <file>] [-c] [-p]

A lightweight dotenv-reading command runner

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

## Usage

If your application needs a bunch of environment variables set, you can create a .env file and set them in the following format:

```sh
$ head -3 ese.env
DB_HOST=localhost
DB_USER=root
DB_PASS=very.good.password.123
```

Now you can start your project with `ese` and it'll load up these env. variables for you:

```sh
ese ./run.sh

# or
ese cargo run

# or 
ese whatever!
```