#![no_std]
#![no_main]

extern crate alloc;

use metro::prelude::*;

struct GameState;

metro_main! {
	|_metro| {
		GameState
	},
	|_state, _metro| {
		MetroStatus::Continue
	}
}