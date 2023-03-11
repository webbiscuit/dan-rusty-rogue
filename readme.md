# Dan's Roguelike Thing

A roguelike game. Mostly to see how Rust and WASM works.
WIP!

## Demo

A demo is available here: https://webbiscuit.github.io/dan-rusty-rogue/


## Build for the web

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/dan-rusty-rogue.wasm --out-dir wasm --no-modules --no-typescript
```
or use trunk

```bash
trunk build --release --public-url dan-rusty-rogue
```