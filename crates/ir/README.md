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

Intermediate layer to restructure an `Upper` layer so that `if`-statements, `while`-loops and `for`-loops are getting flattened.

* Variables
	* Lifetimes
* No specific types for operations
* Flat structure
* Operation-embedded, flat control flow structures (`if`, `while`, `for`)
* Branch coordinates

## `Lower`

Assembly-like layer with bare instructions.

Only extra feature is branching to compile-time dynamic code-coordinates (external functions and dynamic offsets).

* Registers
* Stack
* Bare, fixed-type instructions
* Flat structure
* Branch coordinates

# Conversion

> **Warning**
> 
> **Work-in-progress**

## `Upper` to `Destructure`

### `Condition`

This block has the following labels:

* `after` $\rightarrow$ success
* `cancel` $\rightarrow$ failure

```
(or (> :c 0) (< :c 5))
->
// short-circuit
branchif > :c 0 after
branchif < :c 5 after
// failure
branch cancel
```
---
```
(and (> :c 0) (< :c 5))
->
// inverted conditions, short-circuit
branchif <= :c 0 cancel
branchif >= :c 5 cancel
// success
branch after
```
---
```
(or (x) (y))
->
expand(x)
label cancel ~ for expand(x)
expand(y)
label after ~ for expand(x)
branch after
```
---
```
(and (x) (y))
->
expand(x)
expand(y)
branch after
```
---
```
(if cond code)
->
expand(cond)
label after ~ for expand(cond)
expand(code)
label cancel ~ for expand(cond)
```
---
```
(while cond code)
->
branch check
label after ~ for expand(cond)
expand(code)
label check
expand(cond)
label cancel ~ for expand(cond)
```