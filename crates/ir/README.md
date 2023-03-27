# Layers

Conversion happens from an upper layer down to the lowest layer through the layer(s) in-between.

## `Upper`

Layer expected to be used by high-level languages.

* Variables
* No specific types for operations
* Multi layer structure
* Separate control flow structures (`if`, `while`, `for`)

## `Flatten`

Intermediate layer to flatten an `Upper` layer to later compile down to a `Lower` layer.

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
