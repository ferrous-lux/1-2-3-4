# 1-2-3-4 — Agent Instructions

## Project Overview

A WebAssembly puzzle game written in Rust. The game is a 2×2 grid where the
player fills in numbers 1–4. Each number used exactly once. Odd-numbered
squares must touch other odd-numbered squares (adjacency, not diagonal).

Four modes: Easy (3 prefilled), Medium (2 prefilled), Hard (1 prefilled),
Extreme (0 prefilled).

The frontend is built with Yew (or a similar Rust WASM framework) and
compiled to WebAssembly via `wasm-pack`. The backend is all client-side WASM.

---

## Build / Lint / Test Commands

```bash
# Build (debug)
cargo build

# Build WASM bundle
wasm-pack build --target web

# Build WASM in release mode
wasm-pack build --target web --release

# Run all Rust tests
cargo test

# Run a single test by name (substring match)
cargo test my_test_name

# Run a single test with full stdout
cargo test my_test_name -- --nocapture

# Run WASM tests in headless browser (requires wasm-bindgen-test)
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome
wasm-pack test --headless --safari

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --all

# Check formatting (CI)
cargo fmt --all -- --check

# Type-check (cargo check is the type-check equivalent in Rust)
cargo check
cargo check --target wasm32-unknown-unknown

# Audit dependencies for vulnerabilities
cargo audit
```

**CI should run**: `cargo fmt --check`, `cargo clippy -- -D warnings`,
`cargo test`, and `wasm-pack test --headless --firefox`.

---

## Code Style Guidelines

### Imports

- Group imports in this order, separated by blank lines:
  1. `std` / `core` / `alloc`
  2. External crates
  3. `crate` internal modules
- Use `use` for modules, not full paths in code.
- Prefer `use crate::module::Item` over `use super::*`.
- Import enums' variants explicitly when used more than once.

**Example:**

```rust
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::game::state::GameState;
use crate::game::Mode;
```

### Formatting

- Use `cargo fmt` defaults (4-space indent, 100-char line width as set in
  rustfmt.toml if present; otherwise stick with 100 chars).
- One blank line between function/item definitions.
- No trailing whitespace.
- Terminate all files with a single newline.

### Naming

| Element        | Convention              | Example              |
|----------------|-------------------------|----------------------|
| Types / enums  | `UpperCamelCase`        | `GameState`          |
| Traits         | `UpperCamelCase`        | `Renderable`         |
| Functions      | `snake_case`            | `check_valid()`      |
| Variables      | `snake_case`            | `prefilled_count`    |
| Constants      | `SCREAMING_SNAKE_CASE`  | `MAX_GRID_SIZE`      |
| Modules        | `snake_case`            | `mod game_logic`     |
| Macros         | `snake_case`            | `grid_coords!()`     |
| Error types    | `UpperCamelCase` ending | `ValidationError`    |
| Type params    | short `UpperCamelCase`  | `<T>`, `<E>`         |

- Avoid abbreviations unless they are universal (`idx`, `col`, `row`).
- Boolean variables should be prefixed with `is_`, `has_`, or `should_`.
- Private helpers that return `Result` should be named `try_*` when there is
  also an infallible variant.

### Error Handling

- Define domain errors with `thiserror`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("duplicate number {0}")]
    Duplicate(u8),
    #[error("odd squares must be adjacent")]
    OddNotAdjacent,
}
```

- Use `anyhow` for binary/top-level error handling, `thiserror` for library
  crate errors.
- Prefer `Result<_, GameError>` over panicking. Use `.context()` from `anyhow`
  to annotate errors.
- `unwrap()` / `expect()` are only acceptable in tests or when the invariant
  is provably unreachable (document *why*).

### Types

- Prefer `u8` for grid cell values (1–4 fits in a byte).
- Use `struct Grid([[u8; 2]; 2])` or a newtype for the 2×2 grid.
- Use enums for modes:

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Easy,
    Medium,
    Hard,
    Extreme,
}
```

- Prefer `impl Into<…>` / `impl From<…>` for conversions over manual methods.
- Mark types that represent a fixed set of variants with `#[non_exhaustive]`
  only if the set may grow.
- Derive `Default`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Debug` on data types
  whenever sensible.

### Structs and Methods

- Keep data and behaviour separate: data in structs, behaviour in `impl`
  blocks or separate functions.
- Use `&self` / `&mut self` for methods, free functions when there is no clear
  receiver.
- Constructor: `new()` for the primary constructor; `with_*` for builder
  variants.

```rust
impl Grid {
    pub fn new() -> Self { /* … */ }
}
```

### Pattern Matching

- Exhaustive matches on enums; no wildcards when all variants are known.
- For `Option`, use `?`, `unwrap_or_else`, or `if let` — avoid bare
  `unwrap()` outside tests.
- Prefer `matches!()` macro for boolean checks that don't need destructuring.

### WASM / Frontend

- All `#[wasm_bindgen]` exports are thin wrappers over pure-Rust logic.
- Keep `wasm_bindgen` types (`JsValue`, `Closure`) isolated to boundary
  modules.
- Use `web-sys` / `js-sys` crates for DOM/browser APIs rather than raw FFI.
- Yew components: follow the functional component pattern with hooks.

```rust
#[function_component]
fn GameBoard() -> Html {
    let state = use_state(|| Grid::new());
    html! { /* … */ }
}
```

- WASM panic hook should be installed early: `console_error_panic_hook::set_once()`.

### Tests

- Unit tests go in a `#[cfg(test)] mod tests` at the bottom of each source
  file.
- Integration tests (multiple-module scenarios) go in `tests/` directory.
- Use `wasm_bindgen_test::wasm_bindgen_test` for WASM-specific tests.
- Test naming: `#[test] fn test_<thing_being_tested>()`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_accepts_valid_layout() {
        let grid = Grid::from([[1, 2], [3, 4]]);
        assert!(grid.is_valid());
    }
}
```

### Misc

- No `unsafe` code unless absolutely required; document safety invariants
  with `// SAFETY:`.
- Prefer iterator combinators (`map`, `filter`, `all`, `any`) over explicit
  loops where readability is not harmed.
- Use `#[derive(Debug)]` on all public types for testability.
- Keep modules small (<500 lines); extract large modules into submodules.
- Write doc comments (`///`) on all public API items (`pub fn`, `pub struct`,
  `pub trait`).
- Use `#![deny(missing_docs)]` on library crates; `#![allow(missing_docs)]`
  on binary crates / WASM entry points.
- Dependencies: add a comment next to each entry in `Cargo.toml` that
  briefly states *why* it is needed.
