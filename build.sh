set -e
cargo fmt
cargo build -p game --target wasm32-unknown-unknown
# cargo build --release -p game --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/game.wasm shell/metra-game.wasm
# cp target/wasm32-unknown-unknown/release/game.wasm shell/metra-game.wasm