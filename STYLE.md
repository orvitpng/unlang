# unlang style guide

This document defines mandatory coding standards. Automated enforcement is
planned for future implementation.

## Imports
- Import all traits (e.g., `std::io::Read`).
- Import non-trait items only when used multiple times within the code.
  Otherwise, reference items directly.
- If multiple items from a module are used, import the module.
- Direct import of struct-associated functions is prohibited.
- Group imports from the same module within curly braces {}.
- The order of imports should follow ascending ascii values.

## Formatting
- Lines must not exceed 80 characters in length unless absolutely necessary.
- Use 4 spaces for each indentation level.
- Minimize unnecessary whitespace.