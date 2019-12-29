
# ProjectEuler

Solutions to Project Euler Problems

This repository contains my solutions to the problems found on [Project Euler](https://www.projecteuler.net). The solution to each problem can be found in its own folder, along with a description of the problem and, if necessary, of the solution.

## Languages

The solutions are (or will be) written variously in C, C++, Rust, Python, or Fortran, and sometimes more than one.

Should any of the build instruction for a project differ from those given below, it will be noted in that project's READEME.

### C and C++

All C and C++ solutions are built with [MSVC](https://docs.microsoft.com/en-us/cpp/?view=vs-2019). For C++, this is with:

```shell
cl /EHsc /std:c++17 ./*.cpp
```

And C with:

```shell
cl ./*.cpp
```

### Rust

[Rust](https://www.rust-lang.org/) solutions are simply built with:

```shell
rustc <problem>.rs
```

### Python

All Python solutions are executed with [IPython](https://ipython.org).

```shell
ipython <project>.py
```

### Fortran

Fortran solution are built with [MinGW-w64](http://mingw-w64.org/doku.php), [specifically this version](https://sourceforge.net/projects/mingw-w64/files/Toolchains%20targetting%20Win64/Personal%20Builds/mingw-builds/8.1.0/threads-posix/seh), using the following command:

```shell
gfortran -ffree-form -o <project> *.f
```
