# Rust/WASM Cross-Stitch Module

A Rust library, compiled to WebAssembly, that turns a photo into a cross-stitch pattern: a grid of stitches, each assigned to a DMC thread, plus a legend with thread codes and stitch counts.

> **Status: work in progress.** The pipeline works end to end, but the importance map is not wired in yet and pattern quality has not been evaluated systematically.

**Polish step-by-step guide (clone, build, run, use): [docs/INSTRUKCJA.md](docs/INSTRUKCJA.md)**

## How it works

```
photo (bytes)
  -> resize to (grid size x k)        k = working pixels per stitch
  -> aggregate k x k blocks           one averaged pixel per stitch
  -> color quantization (imagequant)  limits the number of colors
  -> map to the DMC palette           nearest DMC color (Oklab)
  -> majority filter 3x3 on indices   removes isolated single stitches
  -> pattern: grid of indices + legend
```

The canvas size can be given in stitches or in centimeters. For centimeters the module uses the canvas count (stitches per inch): `stitches = round(cm * count / 2.54)`.

## Layout

| Path | Purpose |
|---|---|
| `src/processing/` | the pipeline, plain Rust, testable natively |
| `src/wasm_bridge.rs` | `generate_pattern` exported to JavaScript (`wasm32` only) |
| `src/main.rs` | command-line runner for quick experiments |
| `dmc_palette.csv` | DMC palette, embedded into the `.wasm` at compile time |
| `index.html` | demo page: upload a photo, tweak settings, live preview |

## Quick start

Requirements: Rust 1.85 or newer (the crate uses edition 2024), the `wasm32-unknown-unknown` target and `wasm-pack`.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

wasm-pack build --target web
python3 -m http.server 8000
```

Open `http://localhost:8000`. The page must be served over HTTP, opening the file directly does not work.

Tests for the native part: `cargo test --lib`.

## JavaScript API

```js
import init, { generate_pattern } from './pkg/prototype.js';
await init();

const pattern = generate_pattern(
  imageBytes,   // Uint8Array, a PNG or JPEG file
  true,         // unit_is_cm: true = centimeters, false = stitches
  20,           // width in the chosen unit
  0,            // height in the chosen unit, 0 = from the photo proportions
  14,           // density_per_inch (canvas count), used only for centimeters
  30,           // requested number of colors
  8             // k, working pixels per stitch
);

pattern.width();        // stitches
pattern.height();       // stitches
pattern.indices();      // Uint16Array, one legend index per stitch, row by row
pattern.legend_json();  // [{"code","name","rgb":[r,g,b],"count"}, ...]
pattern.free();         // release WASM memory when done
```

Errors are thrown as JavaScript exceptions with a text message.

## Known limitations

- The number of colors in the legend can be lower than requested: several quantized colors may map to the same DMC thread.
- There are no limits on size or `k` yet. Very large patterns can use a lot of memory in the browser.
- The importance map (`src/processing/importance_map.rs`) exists but is not part of the pipeline.

## License

The repository is MIT licensed. Note that `imagequant` is licensed under GPL-3.0-or-later, which may affect how a compiled build can be distributed. Clarify this before publishing the project.
