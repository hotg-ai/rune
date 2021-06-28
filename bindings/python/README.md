# Rune Python Bindings

A Python module which exposes
A python module exporting the several common proc blocks as a Python module,
primarily intended for use in training models.

## Getting Started (Developer)

If you want to start hacking on the Python bindings, you will need to create a
new virtual environment and install [`maturin`][maturin].

```console
$ cd proc-blocks/python
$ python3 -m venv env
$ source ./env/bin/activate
$ pip install maturin
$ maturin help
maturin 0.9.4
Build and publish crates with pyo3, rust-cpython and cffi bindings as well as rust binaries as python packages

USAGE:
    maturin <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build          Build the crate into python packages
    develop        Installs the crate as module in the current virtualenv
    help           Prints this message or the help of the given subcommand(s)
    list-python    Searches and lists the available python installations
    publish        Build and publish the crate as python packages to pypi
    sdist          Build only a source distribution (sdist) without compiling
```

You can use `maturin develop` to install the bindings into your virtual
environment.

```console
$ maturin develop
🔗 Found pyo3 bindings
🐍 Found CPython 3.9 at python
   Compiling pyo3 v0.13.2
   Compiling proc-blocks v0.1.0 (/home/michael/Documents/hotg-ai/rune/proc-blocks/python)
    Finished dev [unoptimized + debuginfo] target(s) in 3.63s
```

From there, the `proc-blocks` package should be accessible inside your Python
installation.

```console
$ ipython
Python 3.9.2 (default, Feb 20 2021, 18:40:11)
Type 'copyright', 'credits' or 'license' for more information
IPython 7.19.0 -- An enhanced Interactive Python. Type '?' for help.

In [1]: from proc-blocks import Normalize

In [2]: Normalize?
Init signature: Normalize(self, /, *args, **kwargs)
Docstring:      <no docstring>
Type:           type
Subclasses:

In [3]: norm = Normalize()

In [4]: norm([0, 1, 2, 3, 4, 5])
Out[4]: [0.0, 0.2, 0.4, 0.6, 0.8, 1.0]
```

You can use `maturin build` to generate Python Wheels that can be installed
on an end user's machine.

```console
$ maturin build
🔗 Found pyo3 bindings
🐍 Found CPython 3.9 at python3.9
   Compiling pyo3 v0.13.2
   Compiling proc-blocks v0.1.0 (/home/michael/Documents/hotg-ai/rune/proc-blocks/python)
    Finished dev [unoptimized + debuginfo] target(s) in 3.62s
📦 Built wheel for CPython 3.9 to /home/michael/Documents/hotg-ai/rune/target/wheels/proc-blocks-0.1.0-cp39-cp39-manylinux2010_x86_64.whl

$ ls -la ../../target/wheels
.rw-r--r-- 2.7M michael  2 Apr  0:47 proc-blocks-0.1.0-cp39-cp39-manylinux2010_x86_64.whl
```

[maturin]: https://github.com/PyO3/maturin
