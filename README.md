# Leviathan

> **Warning**  
> This branch is highly work-in-progress and in no means stable

Leviathan is a Lisp-inspired programming language.
It is a toy project and not meant for serious use cases.

Leviathan compiles to bytecode for [Urban engine](https://github.com/sqyyy-jar/urban-engine/).

The language currently only supports an assembler mode but will also support a dialect with variables etc. in the future.

## Assembler

### Example program

```clj
; main.lvt
(mod assembly)

(-label main (do
  (halt)
))
```

This program will immediately halt the runtime.
