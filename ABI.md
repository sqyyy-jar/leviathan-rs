# Application Binary Interface

* Stack is **always** same size before and after a call
* Copy values to the top of the stack for on-stack passing
* Order arguments last to fist for on-stack passing (first argument is on top)
* Pre-allocate stack-memory for on-stack return values
* Registers 0 to 15 are **always** non-persistent
* Registers 16 to 31 are **always** persistent

# JIT

* Replace native stack with context stack

## Mapped registers

> **Warning**  
> TODO

The following registers are mapped:

* `rsp` <-> `r31` (stack pointer)

## Call switches

### Virtual => Virtual

* Push return address onto callstack
* Jump to virtual callee address

### Virtual => Native

> **Warning**  
> TODO

* Push return address onto callstack
* Push runner return function address onto stack
* Set first argument to context pointer
* Copy mapped virtual registers to native registers
* Jump to native callee address

### Native => Virtual

> **Warning**  
> TODO

* Load native callee address
* Push return address onto stack (`call`)
* Jump to native callee address (`call`)

Virtual call stub:

* Pop return address and push it onto callstack
* Copy mapped native registers to virtual registers
* Jump to virtual address
  * Set virtual program counter
  * Jump to runner

### Native => Native

> **Warning**  
> TODO

* Load native callee address
* Push return address onto stack (`call`)
* Jump to native callee address (`call`)

## Return switches

### Virtual => Virtual

* Pop return address from callstack
* Jump to virtual return address

### Virtual => Native

> **Warning**  
> TODO

* Pop return address from callstack
* Set first argument to context pointer
* Copy mapped virtual registers to native registers
* Jump to native return address

### Native => Virtual

> **Warning**  
> TODO

* Pop return address from stack (`ret`)
* Jump to native return address (`ret`)

Runner return function:

* Pop return address from callstack
* Jump to virtual return address

### Native => Native

> **Warning**  
> TODO

* Pop return address from stack (`ret`)
* Jump to native return address (`ret`)

# To do

* Add copy instructions
  * `xcpy Xbase i11 i11`: copy 128 bits
  * `ycpy Xbase i11 i11`: copy 256 bits
  * (`zcpy Xbase i11 i11`: copy 512 bits)
