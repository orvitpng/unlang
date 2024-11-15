# 1. Introduction

This specification delineates the design for the first version of unlang (v0.1.0), a functional programming language that synthesizes the best attributes of other languages including Gleam, Erlang, Rust, and Zig. Unlang aims to be a robust general-purpose language prioritizing scalability, safety, and maintainability. The language will be implemented in Rust.

Unlang embraces many language paradigms: [functional programming](https://en.wikipedia.org/wiki/Functional_programming)
with immutability at its core; [concurrent programming](https://en.wikipedia.org/wiki/Concurrent_computing) through built-in support for lightweight processes and messaging; elements of [object-oriented programming](https://en.wikipedia.org/wiki/Object-oriented_programming) design in how it organizes code (albeit without many traditional features like inheritance); and [declarative programming](https://en.wikipedia.org/wiki/Declarative_programming).

# 2. Core Language Features

## 2.1 Lines and Comments

The language does not utilize semicolons (`;`). This is because in unlang, the end of a statement is signaled by the start of a new line.

Regarding comments, only single-line comments are supported. These are denoted with `//` and extend to the end of the current line, with no alternative comment syntax available.

## 2.2 Values

### Type Inferrence 

When defining variables or determining the response type of a function, the type will always be inferred. This is because the language should never have enough ambiguity for the compiler to not know what your type is. Therefore, it is impossible to specify a type for a function or variable.

### Constants

At the top level, constants may be defined using the keyword `given`. These constants are immutable values that can be accessed across modules when declared public (we will address this later).

```unlang
given pi = 3.14159
given gravity = 9.81
```

Constants have specific rules about their usage and placement within code. They can only be defined at the top level of a module and cannot be reassigned or modified after declaration.

```unlang
given foo = 0
// compile-time error: constants cannot be redefined
given foo = 1

pub fn main() {
    // compile-time error: constants cannot be defined outside the module scope
    given bar = 0

    // compile-time error: constants cannot be shadowed
    let foo = 2
}
```

Constants should only be used to store data that will not change throughout the program's execution. They provide a way to define fixed values that can be referenced throughout your code.

The choice of the keyword `given` aligns with mathematical notation and reasoning. In mathematics, one would say "given pi is equal to 3.14..." followed by "let x be equal pi squared." This maintains familiar mathematical semantics in the language. Further, alternatives like "const" emphasizes the technical implementation detail of immutaability rather than the logical role of the value as a starting assumption or known quantity.

### Variables

Variables are declared using the `let` keyword and can only be defined outside of the top level. Each variable is bound to its declaring scope and maintains immutability within that scope.

```unlang
pub fn main() {
    let foo = 0
    foo // 0

    // compile-time error: variables cannot be mutated
    foo = 1
}
```

While variables cannot be mutated, they can be shadowed by new declarations. The shadowed value exists only within its scope, leaving the outer value unchanged when the scope ends.

```unlang
pub fn main() {
    let foo = 0
    
    {
        let foo = 1
        foo // 1
    }
    foo // 0
}
```

This behavior provides a way to work with values that might need different representations in different parts of your code, while maintaining the safety of immutability. Each new declaration creates a distinct variable, even if it shares a name with one from an outer scope.

### Functions

Idk what to do here yet. This is rough
