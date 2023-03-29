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

TypeScript example code:

```ts
type Structure = If | While  
type Cond = NotCond | AndCond | OrCond | ComparisonCond  
  
class Env {  
    coordsIndex = 0  
    ops = <string[]>[]  
  
    allocCoord(): number {  
        return this.coordsIndex++  
    }  
  
    putCoord(coord: number) {  
        this.ops.push(`#${coord}`)  
    }  
  
    branch(coord: number) {  
        this.ops.push(`branch(#${coord})`)  
    }  
  
    branchIf(op: ComparisonCond, coord: number) {  
        this.ops.push(`branchif(${op.left} ${op.op.symbol} ${op.right}, #${coord})`)  
    }  
  
    expandCode(code: Code) {  
        this.ops.push('expand(code)')  
    }  
  
    dump() {  
        for (let op of this.ops) {  
            console.log(op)  
        }  
    }  
}  
  
class If {  
    get type() {  
        return 'if'  
    }  
  
    cond: Cond  
    code: Code  
  
    constructor(left: Cond, right: Code) {  
        this.cond = left  
        this.code = right  
    }  
  
    expand(env: Env) {  
        const success = env.allocCoord()  
        const failure = env.allocCoord()  
        this.cond.expand(env, success, failure, {next: 'success'})  
        env.putCoord(success)  
        env.expandCode(this.code)  
        env.putCoord(failure)  
    }  
}  
  
class While {  
    get type() {  
        return 'while'  
    }  
  
    cond: Cond  
    code: Code  
  
    constructor(left: Cond, right: Code) {  
        this.cond = left  
        this.code = right  
    }  
  
    expand(env: Env) {  
        const check = env.allocCoord()  
        const success = env.allocCoord()  
        const failure = env.allocCoord()  
        env.branch(check)  
        env.putCoord(success)  
        env.expandCode(this.code)  
        env.putCoord(check)  
        this.cond.expand(env, success, failure, {next: 'failure'})  
        env.putCoord(failure)  
    }  
}  
  
class NotCond {  
    get type() {  
        return 'not'  
    }  
  
    cond: Cond  
  
    constructor(cond: Cond) {  
        this.cond = cond  
    }  
  
    expand(env: Env, success: number, failure: number, opts: { next: 'success' | 'failure' | undefined }) {  
        switch (opts.next) {  
            case "success":  
                this.cond.expand(env, failure, success, {next: 'failure'})  
                break  
            case "failure":  
                this.cond.expand(env, failure, success, {next: 'success'})  
                break  
            default:  
                this.cond.expand(env, failure, success, {next: undefined})  
                break  
        }  
    }  
}  
  
class AndCond {  
    get type() {  
        return 'and'  
    }  
  
    left: Cond  
    right: Cond  
  
    constructor(left: Cond, right: Cond) {  
        this.left = left  
        this.right = right  
    }  
  
    expand(env: Env, success: number, failure: number, opts: { next: 'success' | 'failure' | undefined }) {  
        const nextSuccess = env.allocCoord()  
        switch (opts.next) {  
            case "success":  
                this.left.expand(env, nextSuccess, failure, {next: 'success'})  
                env.putCoord(nextSuccess)  
                this.right.expand(env, success, failure, {next: 'success'})  
                break  
            case "failure":  
                this.left.expand(env, nextSuccess, failure, {next: 'success'})  
                env.putCoord(nextSuccess)  
                // Compression  
                if (this.right.type === 'compare') {  
                    (this.right as ComparisonCond).expand(env, success, failure, {next: 'failure'})  
                } else {  
                    this.right.expand(env, success, failure, {next: 'success'})  
                    env.branch(success)  
                }  
                break  
            default:  
                this.left.expand(env, nextSuccess, failure, {next: 'success'})  
                env.putCoord(nextSuccess)  
                this.right.expand(env, success, failure, {next: 'success'})  
                env.branch(success)  
                break  
        }  
    }  
}  
  
class OrCond {  
    get type() {  
        return 'or'  
    }  
  
    left: Cond  
    right: Cond  
  
    constructor(left: Cond, right: Cond) {  
        this.left = left  
        this.right = right  
    }  
  
    expand(env: Env, success: number, failure: number, opts: { next: 'success' | 'failure' | undefined }) {  
        switch (opts.next) {  
            case "success":  
                this.left.expand(env, success, failure, {next: 'failure'})  
                // Compression  
                if (this.right.type === 'compare') {  
                    (this.right as ComparisonCond).expand(env, success, failure, {next: 'success'})  
                } else {  
                    this.right.expand(env, success, failure, {next: 'failure'})  
                    env.branch(failure)  
                }  
                break  
            case "failure":  
                this.left.expand(env, success, failure, {next: 'failure'})  
                this.right.expand(env, success, failure, {next: 'failure'})  
                break  
            default:  
                this.left.expand(env, success, failure, {next: 'failure'})  
                this.right.expand(env, success, failure, {next: 'failure'})  
                env.branch(failure)  
                break  
        }  
    }  
}  
  
class ComparisonCond {  
    get type() {  
        return 'compare'  
    }  
  
    op: CompareOp  
    left: number  
    right: number  
  
    constructor(op: CompareOp, left: number, right: number) {  
        this.op = op  
        this.left = left  
        this.right = right  
    }  
  
    inverted(): ComparisonCond {  
        return new ComparisonCond(this.op.inverted(), this.left, this.right)  
    }  
  
    expand(env: Env, success: number, failure: number, opts: { next: 'success' | 'failure' | undefined }) {  
        switch (opts.next) {  
            case "success":  
                env.branchIf(this.inverted(), failure)  
                break  
            case "failure":  
                env.branchIf(this, success)  
                break  
            default:  
                env.branchIf(this, success)  
                env.branch(failure)  
                break  
        }  
    }  
}  
  
type CompareSymbol = '==' | '!=' | '<' | '>' | '<=' | '>='  
  
class CompareOp {  
    symbol: CompareSymbol  
  
    constructor(symbol: CompareSymbol) {  
        this.symbol = symbol  
    }  
  
    inverted(): CompareOp {  
        switch (this.symbol) {  
            case "==":  
                return new CompareOp('!=')  
            case "!=":  
                return new CompareOp('==')  
            case "<":  
                return new CompareOp('>=')  
            case ">":  
                return new CompareOp('<=')  
            case "<=":  
                return new CompareOp('>')  
            case ">=":  
                return new CompareOp('<')  
        }  
    }  
}  
  
class Code {  
}  
  
function expand(env: Env, structure: Structure) {  
    switch (structure.type) {  
        case "if":  
            (structure as If).expand(env)  
            break  
        case "while":  
            (structure as While).expand(env)  
            break  
    }  
}  
  
function $if(cond: Cond, code: Code): Structure {  
    return new If(cond, code)  
}  
  
function $while(cond: Cond, code: Code): Structure {  
    return new While(cond, code)  
}  
  
function $and(left: Cond, right: Cond): Cond {  
    return new AndCond(left, right)  
}  
  
function $or(left: Cond, right: Cond): Cond {  
    return new AndCond(left, right)  
}  
  
function $not(cond: Cond): Cond {  
    return new NotCond(cond)  
}  
  
function $(left: number | Cond, symbol: CompareSymbol | '&&' | '||', right: number | Cond): Cond {  
    switch (symbol) {  
        case "&&":  
            return new AndCond(left as Cond, right as Cond)  
        case "||":  
            return new OrCond(left as Cond, right as Cond)  
        case "==":  
        case "!=":  
        case "<":  
        case ">":  
        case "<=":  
        case ">=":  
            return new ComparisonCond(new CompareOp(symbol as CompareSymbol), left as number, right as number)  
        default:  
            throw new Error("Invalid symbol combination")  
    }  
}  
  
const env = new Env()  
const structure = $if(  
    $(  
        $(  
            $(1, '!=', 1),  
            '&&',  
            $(  
                $(1, '!=', 2),  
                '||',  
                $not($(1, '!=', 3))  
            )  
        ),  
        '&&',  
        $(1, '!=', 4)  
    ),  
    new Code()  
)  
expand(env, structure)  
env.dump()
```