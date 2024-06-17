# bevy-in-leptos
```bash
cd bevy-breakout
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./wasm-bindgen-out \
    --out-name "bevy-breakout" \
    ./target/wasm32-unknown-unknown/release/bevy-breakout.wasm
cd ..
trunk serve
```