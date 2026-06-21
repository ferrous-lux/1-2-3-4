# 1-2-3-4

A 2×2 number puzzle game compiled to WebAssembly.

## Rules

Fill a 2×2 grid with the numbers 1, 2, 3, and 4 exactly once.
Odd-numbered squares (1, 3) must orthogonally touch another odd square.

## Modes

| Mode | Prefilled |
|------|-----------|
| Easy | 3 |
| Medium | 2 |
| Hard | 1 |
| Extreme | 0 |

## Building

```bash
# Build the WASM bundle
wasm-pack build --target web

# Serve locally (any static file server works)
python3 -m http.server 8080
```

Then open `http://localhost:8080` in a browser.

## How it works (the dirty secrets)

We were originally using `rand` to generate random valid grids by shuffling
`[1,2,3,4]` and checking the odd-adjacency rule, retrying until we found a
valid one. Then we realized we could just…

### Precompute all 16 valid grids

There are exactly 16 ways to arrange 1–4 on a 2×2 grid where odd numbers
share an edge. We hardcoded all of them in a static array.

### Replace rand with click counting

We removed the `rand` dependency entirely. The "randomness" comes from
`(click_count + mode_offset) % 16`, which picks both which grid to use and
which cells to blank. The click counter wraps on overflow, so it's
effectively free-running.

| Mode | Grid index | Blank pattern |
|------|-----------|---------------|
| Extreme | N/A | All 4 cells empty |
| Easy/Hard/Medium | `(clicks + mode_offset) % 16` | `grid_idx / 4` selects which cells to blank |

Buttons, mode selection, cell clicks, and "Check Solution" all increment
the counter, so the puzzle changes based on user interaction.

### Validate by set membership

Validating a solution is a single `ALL_VALID_GRIDS.contains(grid)` check.
No rule evaluation needed — we just check if the completed grid is one of
the 16 known-good arrangements.

### Remove the framework

We started with Yew (a Rust WASM framework) but the UI is just 4 cells and
6 buttons, so we replaced it with direct `web-sys` DOM calls. This dropped
the WASM binary from 178 KB to 32 KB and removed 6 dependencies.

## Current dependencies

Only 3 runtime dependencies:

- `wasm-bindgen`: WASM/JS glue
- `web-sys`: DOM bindings
- `console_error_panic_hook`: readable panics in the browser console

## License

MIT
