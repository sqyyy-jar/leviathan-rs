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
(or (> :c 0) (< :c 5)) // wrong
->
// short-circuit
branchif > :c 0 after
branchif < :c 5 after
// failure
branch cancel
```
---
```
(and (> :c 0) (< :c 5)) // wrong
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
1: expand(x)
2: label cancel ~ for 1
3: expand(y)
4: label after ~ for 1
5: branch after
```
---
```
(and (x) (y))
->
1: expand(x)
2: label after ~ for 1
3: expand(y)
4: branch after
```
---
```
(if cond code)
->
1: expand(cond)
2: label after ~ for 1
3: expand(code)
4: label cancel ~ for 1
```
---
```
(while cond code)
->
1: branch check
2: label after ~ for 5
3: expand(code)
4: label check
5: expand(cond)
6: label cancel ~ for 5
```

#### Examples

```
(if (and (or (< :c 0) (> :c 100)) (!= :d 0)) {
  (return)
})
===>
// <if>
1: expand(cond)
2: label after ~ for 1
3: expand(code)
4: label cancel ~ for 1
// </if>
===>
// <if>
// <if.cond>
1: expand(cond.left)
2: label after ~ for 1
3: expand(cond.right)
4: branch after{5}
// </if.cond>
5: label after ~ for 2..4
6: expand(code)
7: label cancel ~ for 2..4
// </if>
===>
// <if>
// <if.cond>
// <if.cond.left>
1: expand(cond.left.left)
2: label cancel ~ for 1
3: expand(cond.left.right)
4: label after ~ for 1
5: branch after{6}
// </if.cond.left>
6: label after ~ for 1..5
// <if.cond.right>
7: branchif != :d 0 after{6}
8: branch cancel{12}
// </if.cond.right>
9: branch after{10}
// </if.cond>
10: label after ~ for 6..9
11: expand(code)
12: label cancel ~ for 6..9
// </if>
```

### Other concept

This concept is written in TypeScript pseudo code.

```ts
function expandIf(stmnt: IfStmnt) {
	if (stmnt.cond.is_direct()) {
		// let success = allocCoord()
		let failure = allocCoord()
		emit(
			branchIf(
				stmnt.cond.op.inverted(),
				stmnt.cond.left,
				stmnd.cond.right,
				failure,
			)
		)
		// putCoord(success)
		emit(expand(stmnt.code))
		putCoord(failure)
	} else {
		let success = allocCoord()
		let failure = allocCoord()
		emit(expandCond(stmnt.cond, success, failure))
		putCoord(success)
		emit(expandCode(stmnt.code))
		putCoord(failure)
	}
}

function expandCond(cond: Cond, success: Coord, failure: Coord) {
	// TODO
}
```