# Layers

Conversion happens from an upper layer down to the lowest layer through the layer(s) in-between.

## `Upper`

Layer expected to be used by high-level languages.

This layer is required to check bounds by itself. Later layers do not have the ability to report meaningful errors.

* Variables
* No specific types for operations
* Multi layer structure
* Separate control flow structures (`if`, `while`, `for`)

## `Destructure`

Intermediate layer to destructure an `Upper` layer so that `if`-statements, `while`-loops and `for`-loops are inlined.

* Variables
	* Define
	* Undefine
	* Lifetimes
* No specific types for operations
* Flat structure
* Operation-embedded, flat control flow structures (`if`, `while`, `for`)
* Branch coords

## `Lower`

Assembly-like layer with bare instructions.

Only extra feature is branching to compile-time dynamic code-coordiantes (extern functions and dynamic offsets).

* Registers
* Stack
* Fixed types for operations
* Flat structure
* Branch coords