# Leviathan

Leviathan is a Lisp-inspired programming language.
It is a toy project and not meant for serious use cases.

Leviathan compiles to bytecode for [Urban engine](https://github.com/sqyyy-jar/urban-engine/).

The language currently only supports an assembler dialect but will also support a more comfortable dialect with variables etc. in the future.

## Assembler

### Example program

```clj
; main.lvt
(mod assembly)

(static message "Hello world!\n")

(-label main (do
  (mov r0 1u) ; move 1 into r0 (file descriptor)
  (ref r1 message) ; load address of message into r1
  (ldr r2 r1 -8) ; load the length of the string into r2
  (int 1u) ; write interrupt
  (halt) ; halt the process
))
```

## Project

A project consists of:

* A `build.lvt.toml` file with the build configuration
* A `src` directory with the source files
* The `main.lvt` source file in the `src` directory with a `main` function
* Other source files in the `src` directory

### Config

The build configuration looks like this:

```toml
[package]
name = "my-package"
version = "1.0.0"
```