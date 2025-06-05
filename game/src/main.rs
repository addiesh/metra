#![no_std]
#![no_main]

extern crate alloc;

use log::{debug, error, info, trace, warn};
use metro::prelude::*;

struct GameState;

fn init(engine: &mut Metro) -> GameState {
	trace!("hello from init!");
	error!("hello from init!");
	warn!("hello from init!");
	info!("hello from init!");
	debug!("hello from init!");

	info!("random value: {}", engine.random());
	if !engine.save_persistent("skibidi".as_bytes()) {
		error!("failed to save persistent data :(")
	}
	assert_eq!(*engine.load_persistent().unwrap(), *"skibidi".as_bytes());
	GameState
}

fn update(_state: &mut GameState, _engine: &mut Metro) -> MetroStatus {
	// info!("update");
	MetroStatus::Continue
}

metro_main! {
	init,
	update
}
