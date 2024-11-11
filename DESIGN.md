# unlang

## 1. Introduction

This document outlines the design for the first version of unlang (v0.1.0). It
is designed as a modern functional programming language, aiming combine the
best features of other languages, such as Gleam, Erlang, Rust, and Zig. It will
be a robust general-purpose language that is a highly-scalable, safe, and
maintainable environment for developers. Unlang will be implemented in Rust,
leveraging its high performance and low-level control.

Unlang embraces many language paradigms:
[functional programming](https://en.wikipedia.org/wiki/Functional_programming)
with immutability at its core; elements of
[concurrent programming](https://en.wikipedia.org/wiki/Concurrent_computing)
through built-in support for lightweight processes and messaging; elements of
[object-oriented programming](https://en.wikipedia.org/wiki/Object-oriented_programming)
in its module system and type design, although many traditional features like
inheritance/polymorphism are not included; and
[declarative programming](https://en.wikipedia.org/wiki/Declarative_programming).

This document aims to outline the design of unlang and serve as a comprehensive
guide for a future implementation. As of now, no code has been written, and
this design is subject to extreme change based on challenges in implementation,
feedback, and what aligns with the goals of this language.

## 2. Core Language Features

### 2.1 Functions

Functions are first-class citizens in unlang, meaning they can be assigned to
variables, passed as arguments, and returned from other functions. They are
defined with the `fn` keyword and use `=` to separate the signature from its
body, similar to how most languages would define their variables.

#### Basic function syntax

```unlang
fn add(a: Int, b: Int) = a + b
```

Functions with multiple expressions would use a block:

```unlang
fn complex_calculation(a: Int, b: Int) = {
  let c = x * y
  let d = c + 10
  d * 2
}
```

#### Anonymous Functions

Anonymous functions (lambdas) use a similar syntax, just without the `fn`
keyword:

```unlang
let multiply = (a: Int, b: Int) = a *b
```

#### Higher-order Functions

Functions can take other functions as arguments:

```unlang
fn apply_twice(fun: (Int) -> Int, a: Int) = fun(fun(a))
```

They can also return a function:

```unlang
fn create_multiplier(factor: Int) = (a: Int) = a * factor
```

### 2.2 Variables and Constants

Unlang uses `let` for rebindable variables and `const` for constants set at the
top level. All variables are immutable by default, promoting safer and more
predictable code by preventing unintended modifications.

```unlang
let a = 5 // Not allowed; compile-time error due to `let` at the top level

const b = 5 // Allowed; b = 5
const b = b + 5 // Not allowed; compile-time error due to rebinding constant
```

```unlang
fn foo() = {
  let a = 5 // Allowed; a = 5
  let a = a + 5 // Allowed; a = 10

  const b = 5 // Not allowed; compile-time error due to `const` not at the top level
}
```

### 2.3 Types

Unlang has a strong static type system with inference. Types are defined using
the `type` keyword.

#### Basic Types

```unlang
type Name = String
type Age = Int
```

#### Type Aliases

```unlang
type IntPair = (Int, Int)
type StringList = List(String)
```

#### Generic Types

```unlang
type Pair(a, b) = {
  first: a,
  second: b,
}
```

#### Struct Types

```unlang
type User = {
  name: String,
  age: Int,
}
```

#### Enum Types

```unlang
type Option(a) = Some(a) | None

type Result(value, error) = Ok(value) | Error(error)

type Shape =
  | Circle(radius: Int)
  | Rectangle(width: Int, height: Int)
  | Triangle(base: Int, height: Int)
```

### 2.4 Pattern Matching

Pattern matching is the most important, and one of the only, ways to manage
control flow in unlang. It can also be used for destructuring.

#### Basic Pattern Matching

```unlang
import std/int

fn describe_point(point: (Int, Int)) = {
  case point {
    (0, 0) = "Origin"
    (0, y) = "On y-axis at " <> int.to_string(y)
    (x, 0) = "On x-axis at " <> int.to_string(x)
    (x, y) =
      "Point at (" <> int.to_string(x) <> ", " <> int.to_string(y) <> ")"
  }
}
```

#### Pattern Matching with Enums

```unlang
import std/int

fn process_result(result: Result(Int, String)) = {
  case result {
    Ok(value) = "Success: " <> int.to_string(value)
    Error(msg) = "Error: " <> msg
  }
}
```

#### Pattern Matching Assertions

Unlang allows for pattern matching assertions by using the `assert` keyword:

```unlang
fn get_username(user: Option(User)) = {
  let assert Some(user) = user
  user.name
}

fn process_success(input: String) = {
  let assert "Success: " <> msg = input
  msg
}
```

Pattern matching assertions allow you to combine pattern matching with variable
binding, throwing an error if the pattern doesn't match. This can lead to more
concise and expressive code.

#### Exhaustiveness Checking

The unlang compiler will perform exhaustiveness checking on pattern matches,
ensuring that all possible cases are handled:

```unlang
type TrafficLight = Red | Yellow | Green

fn describe_light(light: TrafficLight) = {
  case light {
    Red = "Stop"
    Yellow = "Caution"
    // Not allowed; compile-time error due to green not checked
  }
}
```

### 2.5 Modules and Imports

Code in unlang is organized into modules. The module name is derived from the
file name, or, if the file is titled `mod`, it is taken from the directory
name.

#### Defining a Module

```unlang
// In `src/std/math/mod.unlang`
pub fn add(a: Int, b: Int) = a + b
pub fn multiply(a: Int, b: Int) = a * b

fn private_helper() = {
    // not visible outside the module
}
```

#### Importing and Using Modules

```unlang
import std/math

fn calc(x: Int, y: Int) = math.add(x, math.multiply(y, 2))
```

You can also import specific functions:

```unlang
import std/math.{add, multiply}

fn calculate(x: Int, y: Int) = add(x, multiply(y, 2))
```

### 2.6 Concurrency

Unlang supports Erlang-style concurrency, meaning lightweight processes and
message passing.

#### Spawning Processes

```unlang
import core/process

type Message =
    | Incr
    | Get(pid: process.Pid)

fn spawn_counter() = {
    process.spawn(() = counter(0))
}

fn counter(count: Int) = {
    case process.receive() {
        Incr = counter(count + 1)
        Get(pid) = {
            process.send(pid, count)
            counter(count)
        }
    }
}
```

#### Sending and Receiving Messages

```unlang
let pid = spawn_counter()
process.send(pid, Incr)
process.send(pid, Get(process.self))
let count = process.receive() // 1
```

### 2.7 Error Handling

Unlang uses the `Result` type for error handling, encouraging explicit error
management.

```unlang
import std/int

// returns Result(Int, String)
fn divide(a: Int, b: Int) = {
    case b {
        0 = Error("Division by zero")
        _ = Ok(a / b)
    }
}

fn use_division = {
    case divide(10, 2) {
        Ok(result) = "Success: " <> int.to_string(result)
        Error(msg) = "Error: " <> msg
    }
}
```

## 3. Syntax Specification

- Identifiers: `[a-z][a-zA-Z0-9_]*` for variables and functions,
  `[A-Z][a-zA-Z0-9_]*` for types and modules.
- Keywords: `const`, `let`, `type`, `fn`, `case`, `import`, `pub`
- Operators: `+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `<>`
  (string concatenation), `|>` (piping)

Continuing on that, unlang supports arbitrary-depth exponentiation operations.
The snytax is as follows:

- `**`: Standard exponentiation (power)
- `***`: Tetration
- `****`: Pentation
- etc (no upper limit)
