# Style Guide

This guide outlines the coding standards for the unlang project. These standards are currently manually enforced, with plans for automated enforcement in the future. Try to adhere to this, but it is not absolute as no human can detect all errors.

## Code Organization

### Imports and Modules

Imports should be organized according to the following rules:

- Always import traits and macros explicitly
- Use fully qualified paths for single-use items
- Import modules when multiple items from that module are used
- Never directly import struct-associated functions
- Group imports from the same module using nested paths (`{}`)
- Sort imports alphabetically, lowercase first
- Place std imports in a separate block above everything else

### Line Length and Formatting

The codebase follows these formatting rules:

- Keep lines to a maximum of 80 characters where possible
- Use 4 spaces for indentation (no tabs)
- Remove trailing whitespace and unnecessary blank lines
- Run `cargo fmt` before committing changes

```rust
// good
let some_var = some_function_call()
    .with_some_method()
    .and_another()
    .and_yet_another_long_method();

// bad
let some_var = some_function_call().with_some_method().and_another().and_yet_another_long_method();
```
