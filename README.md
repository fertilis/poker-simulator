# Poker Simulator Game

## Demo

Wasm: <https://fertilis.github.io/poker-simulator>


## Purpose

Get acquainted with Bevy. This is my first game on Bevy.

## Description

The game is simplistic Texas holdem poker. Opponets move randomly. There is only one raise size.
Apart from this the game is fully functional.

Logic is in `src/table`.

Rendering is in `src/graphics`.

`hand_evaluation.rs` is taken from <https://github.com/elliottneilclark/rs-poker>

## Build

```bash
cargo build -r --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web --out-dir ./web/  --out-name "poker-simulator"  ./target/wasm32-unknown-unknown/release/poker-simulator.wasm
```


