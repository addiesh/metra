// #![no_std]
// extern crate alloc;
//
// use alloc::vec;
// use alloc::vec::Vec;
// use metro::Mesh;
//
// struct GameState<'a> {
// 	vec: Vec<Mesh<'a>>
// }

struct LongestData(Vec<i32>);

struct HoldsLongReference<'a> {
	ld: &'a LongestData
}

impl<'a> HoldsLongReference<'a> {
	fn get_needer(&'a self) -> NeedsLongReference<'a> {
		NeedsLongReference {
			ld: self.ld,
		}
	}
}

struct NeedsLongReference<'a> {
	ld: &'a LongestData
}

struct GivenLongReference<'a> {
	pub needers: Vec<NeedsLongReference<'a>>
}

fn made_generic<'longest, T>(
	longest_data: &'longest LongestData,
	init: fn(&HoldsLongReference<'longest>, lg: &'longest LongestData) -> T,
) {
	let mut holds_long_reference: HoldsLongReference = HoldsLongReference {
		ld: &longest_data
	};

	let given_long_reference = init(&mut holds_long_reference, longest_data);

	drop(given_long_reference);
	drop(holds_long_reference);
}

fn main() {
	let longest_data = LongestData(vec![1, 2, 3]);

	made_generic(
		&longest_data,
		|e, lg| {
			GivenLongReference {
				needers: vec![
					NeedsLongReference { ld: lg },
				],
			}
		},
	);

	// metro::run(
	// 	|metro| {
	// 		let mesh = metro.new_mesh_temp();
	// 		GameState { vec: vec![mesh] }
	// 	},
	// 	|state, metro| {
	// 		println!("context: {}", state.vec[0].context);
	// 		false
	// 	}
	// )
}
