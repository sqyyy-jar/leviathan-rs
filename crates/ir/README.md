# Layers

Conversion happens from an upper layer down to the lowest layer through the layer(s) in-between.

## `Upper`

Layer expected to be used by high-level languages.

This layer is required to check bounds by itself. Later layers do not have the ability to report meaningful errors.

* Variables
* No specific types for operations
* Multi layer structure
* Separate control flow structures (`if`, `while`, `for`)

## Control Flow Flattening (`Destructure`)

Intermediate layer to restructure an `Upper` layer so that `if`-statements, `while`-loops and `for`-loops are getting flattened to conditional branches.

* Variables
	* Lifetimes
* No specific types for operations
* Flat structure
* Operation-embedded, flat control flow structures (`if`, `while`, `for`)
* Branch coordinates

## Constant evaluation (`const_eval`)

> **Warning**  
> Work in progress

This part of code will evaluate expressions that are constant (e.g. addition of two integer literals).

The result of the constant evaluation will be another `Destructure` layer.

## `Lower`

Assembly-like layer with bare instructions.

Only extra feature is branching to compile-time dynamic code-coordinates (external functions and dynamic offsets).

* Registers
* Stack
* Bare, fixed-type instructions
* Flat structure
* Branch coordinates