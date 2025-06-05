use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct KeyboardConfig {}

#[derive(Serialize, Deserialize)]
struct GamepadConfig {}

#[derive(Serialize, Deserialize)]
struct ActionConfig {
	keyboard: KeyboardConfig,
	gamepad: GamepadConfig,
}

fn main() {}
