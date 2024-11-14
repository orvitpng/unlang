# 1. Introduction

This specification delineates the design for the first version of unlang (v0.1.0), a functional programming language that synthesizes the best attributes of other languages including Gleam, Erlang, Rust, and Zig. Unlang aims to be a robust general-purpose language prioritizing scalability, safety, and maintainability. The language will be implemented in Rust.

Unlang embraces many language paradigms: [functional programming](https://en.wikipedia.org/wiki/Functional_programming)
with immutability at its core; [concurrent programming](https://en.wikipedia.org/wiki/Concurrent_computing) through built-in support for lightweight processes and messaging; elements of [object-oriented programming](https://en.wikipedia.org/wiki/Object-oriented_programming) design in how it organizes code (albeit without many traditional features like inheritance); and [declarative programming](https://en.wikipedia.org/wiki/Declarative_programming).

# 2. Core Language Features

## 2.1 Lines and Comments

The language does not utilize semicolons (`;`). This is because in unlang, the end of a statement is signaled by the start of a new line.

Regarding comments, only single-line comments are supported. These are denoted with `//` and extend to the end of the current line, with no alternative comment syntax available.
