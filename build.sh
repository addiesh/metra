set -e
cargo fmt
cargo build --target wasm32-unknown-unknown
# cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/game.wasm shell/metro-game.wasm
# cp target/wasm32-unknown-unknown/release/game.wasm shell/metro-game.wasm