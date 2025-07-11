#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::{String, ToString};
use log::{debug, error, info, trace, warn};
use metra::prelude::*;

struct GameState {
	_the_mesh: Resource<Mesh>,
}

impl Drop for GameState {
	fn drop(&mut self) {
		debug!("game state dropped");
	}
}

fn init(engine: &mut Metra) -> GameState {
	trace!("hello from init!");
	error!("hello from init!");
	warn!("hello from init!");
	info!("hello from init!");
	debug!("hello from init!");

	info!(
		"current persistent data: {:?}",
		engine
			.load_persistent()
			.map(|d| String::from_utf8(d.to_vec()).ok())
			.flatten()
	);
	let random_string = engine.random().to_string();
	info!("random value: {random_string}");
	if !engine.save_persistent(random_string.as_bytes()) {
		error!("failed to save persistent data :(")
	}
	assert_eq!(
		*engine.load_persistent().unwrap(),
		*random_string.as_bytes()
	);
	info!("Passed save/load check!");
	GameState {
		_the_mesh: engine.new_unit_mesh(),
	}
}

fn update(_state: &mut GameState, _engine: &mut Metra) -> MetraStatus {
	// info!("update");
	// _engine.
	// MetraStatus::Continue
	MetraStatus::Stop
}

metra_main! {
	// TODO: fix resource manifests
	include!("manifest"),
	init,
	update
}
