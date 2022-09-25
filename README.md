The template of a web application.

Includes:
* The static web server builded with [axum](https://github.com/tokio-rs/axum).
* The WASM application builded with [seed](https://github.com/seed-rs/seed).
* The `build.rs` script uses `wasm-pack` to produce an assembly.

Use Cargo to build entire project and run the server. Pass `-r`/`--release` flag to build a production ready assembly.
