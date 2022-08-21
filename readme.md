# Dan's Rogue Thing

## Build for the web

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/dan-rusty-rogue.wasm --out-dir wasm --no-modules --no-typescript
