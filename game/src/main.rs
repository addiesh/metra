#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use log::info;
use metro::{Mesh, Metro};

struct GameState {
	vec: Vec<Mesh>,
}

// TODO: this doesn't really make sense. JS calls *in* to WASM, not the other way round.
//		 So I'm not even really sure how to make this work besides evil static functions
//		 or- wait actually yeah let's just do that
#[unsafe(no_mangle)]
fn metro_main() {
	metro::run(
		|metro| {
			// let mesh = metro.new_mesh_temp();
			GameState {
				vec: vec![],
			}
		},
		|state, metro| {
			// info!("context: {}", state.vec[0].context);
			false
		}
	);
}
