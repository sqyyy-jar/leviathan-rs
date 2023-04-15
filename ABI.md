# Application Binary Interface

* Stack is **always** same size before and after a call
* Copy values to the top of the stack for on-stack passing
* Order arguments last to fist for on-stack passing (first argument is on top)
* Pre-allocate stack-memory for on-stack return values
* Registers 0 to 15 are **always** non-persistent
* Registers 16 to 31 are **always** persistent

# JIT

* Replace native stack with context stack

## Call switches

### Mapped registers

> TODO

The following registers are mapped:

* `rsp` <-> `r31` (stack pointer)

### Virtual -> Virtual

* Push return address onto callstack
* Jump to virtual address

### Virtual -> Native

> TODO

* Pop return address
* Push virtual runner return address onto stack
* Set first argument to context pointer
* Copy mapped virtual registers to native registers
* Jump to native address

### Native -> Virtual

> TODO

* Load callee address
* Push return address onto stack (`call`)
* Jump to callee address (`call`)

Virtual call stub:

* Pop return address and push it onto callstack
* Copy mapped native registers to virtual registers
* Jump to virtual address

### Native -> Native

> TODO

* Load native address
* Push return address onto stack (`call`)
* Jump to native address (`call`)

# To do

* Add copy instructions
  * `xcpy Xbase i11 i11`: copy 128 bits
  * `ycpy Xbase i11 i11`: copy 256 bits
  * (`zcpy Xbase i11 i11`: copy 512 bits)
