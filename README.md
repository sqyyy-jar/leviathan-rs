# Urban-engine

Urban-engine is a runtime for a custom binary format with its own instruction set.
It is meant to be a toy project and should not be used for anything serious.

## Archticture

### Registers

The architecture provides 32 registers, 30 of which are for general purpose use.

This table shows the expected usage for each register:

| Register     | Purpose                                                                                                                     |
|:------------:|-----------------------------------------------------------------------------------------------------------------------------|
| `r0 to r7`   | These registers are used for parameter passing between functions. Values need to be saved by the caller.                    |
| `r8 to r15`  | These registers are temporary. Values need to be saved by the caller.                                                       |
| `r16 to r23` | These registers are expected to be saved by the callee.                                                                     |
| `r24 to r29` | These registers do not have any special purpose but may be used in any way.                                                 |
| `r30`        | This register stores the return address to the calling function. It is automatically modified when using labeled branching. |
| `r31`        | This register stores the stack pointer. It may be used for other purposes.                                                  |
### Memory

The memory accessed by the program is the same that is accessed by the runtime,
thus the program could potentially modify runtime values that are not meant to be modified.

Illegal memory access is "handled" by catching segmentation faults
(memory access violations on Windows) and panicking.

The instruction set provides instructions to load the memory offset to the start of the binary.

## CLI

### Installation

For the installation need to have Cargo installed.

To then install the runtime execute the following command:

```bash
cargo install --git https://github.com/sqyyy-jar/urban-engine.git --bin urban
```

This will install the CLI-tool called `urban`.

### Running binaries

To run a binary make sure you can access the binary you want to run in your terminal.

Then just execute the following command:

```bash
urban run ./path/to/binary.bin
```

### Compilation (Deprecated)

The CLI supports some experimental kind of compilation.
It does not really do anything useful and will be removed in the future.

## Writing code

### Leviathan

[Leviathan](https://github.com/sqyyy-jar/leviathan-rs) is a language I made for this runtime.
The language uses a Lisp-style syntax.
It currently only supports an assembler mode but will provide a better dialect in the future.
