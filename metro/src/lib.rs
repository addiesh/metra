#![no_std]
extern crate alloc;

#[global_allocator]
static DLMALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use sys::ModelSys;

mod arena;
mod sys;


// fn _font_render_example() {
// 	let font_data = std::fs::read("FluxischElse-Bold.otf").unwrap();
//
// 	let font = Font::try_from_bytes(&font_data).unwrap();
//
// 	// The font size to use
// 	let scale = Scale::uniform(128.0);
//
// 	// The text to render
// 	let text = "text rendering example";
//
// 	let colour = (0, 0, 0);
//
// 	let v_metrics = font.v_metrics(scale);
//
// 	// layout the glyphs in a line with 20 pixels padding
// 	let glyphs: Vec<_> = font
// 		.layout(text, scale, rusttype::point(20.0, 20.0 + v_metrics.ascent))
// 		.collect();
//
// 	// work out the layout size
// 	let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
// 	let glyphs_width = {
// 		let min_x = glyphs
// 			.first()
// 			.map(|g| g.pixel_bounding_box().unwrap().min.x)
// 			.unwrap();
// 		let max_x = glyphs
// 			.last()
// 			.map(|g| g.pixel_bounding_box().unwrap().max.x)
// 			.unwrap();
// 		(max_x - min_x) as u32
// 	};
//
// 	// Create a new rgba image with some padding
// 	let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();
//
// 	// Loop through the glyphs in the text, positing each one on a line
// 	for glyph in glyphs {
// 		if let Some(bounding_box) = glyph.pixel_bounding_box() {
// 			// Draw the glyph into the image per-pixel by using the draw closure
// 			glyph.draw(|x, y, v| {
// 				image.put_pixel(
// 					// Offset the position by the glyph bounding box
// 					x + bounding_box.min.x as u32,
// 					y + bounding_box.min.y as u32,
// 					// Turn the coverage into an alpha value
// 					Rgba([colour.0, colour.1, colour.2, (v * 255.0) as u8]),
// 				)
// 			});
// 		}
// 	}
//
// 	// Save the image to a png file
// 	image.save("image_example.png").unwrap();
// }

pub struct Metro {
	is_running: bool,
}

impl Metro {
	#[inline]
	pub(crate) fn is_running(&self) -> bool {
		self.is_running
	}

	fn new() -> Self {
		Self {
			is_running: true,
		}
	}

	/// Returns the time elapsed since the start of the game,
	/// in milliseconds.
	pub fn get_time(&self) -> u64 { todo!() }

	pub fn new_mesh(&mut self, model: Asset<Model>, material: Asset<Material>) -> Mesh {
		Mesh {
			internal: Rc::new(MeshInternal {
				model,
				material,
			})
		}
	}

	// pub fn get_action_state<T>(&self) -> { }
	// pub fn did_just_action<T>(&self) -> bool {}
	// pub fn load_asset(&self) -> Asset {}
}


/// The borrow checker is able to enforce most of the rules 
/// regarding lifetimes in the engine, but the most problematic
/// exception is the smart pointer type
struct OwnershipFailsafe {

}

pub struct Asset<T> {
	rc: Rc<T>
}

pub struct Model {
	positions: Box<[[f32; 2]]>,
	coordinates: Box<[[f32; 2]]>,
	indices: Box<[[u16; 3]]>,
	model_internal: ModelSys,
	from_static: bool,
}

struct MeshInternal {
	model: Asset<Model>,
	material: Asset<Material>,
}

pub struct Mesh {
	internal: Rc<MeshInternal>,
}

pub struct Light {}

pub struct Sound {}

pub struct Texture {
	width: u32,
	height: u32,
}

pub struct Material {}

pub struct Effect { }

pub fn run<T>(
	init: fn(&mut Metro) -> T,
	update: fn(state: &mut T, metro: &mut Metro) -> bool
) {
	fn tick<T>(
		game_state: &mut T,
		metro: &mut Metro,
		update: fn(state: &mut T, metro: &mut Metro) -> bool
	) {
		update(game_state, metro);
	}

	// 1. initialize the engine state
	// 2. initialize the game state, provide access to the engine handle to load assets
	// 3. run 1 game frame detached from the remaining game loop
	// 4. show the game window (if applicable)
	// 5. loop game frames until closing
	// 6. hide the game window and free resources

	let mut metro = Metro::new();

	// TODO: initialize the engine state

	let mut game_state = init(&mut metro);

	tick(&mut game_state, &mut metro, update);
	// show window

	while metro.is_running() {
		tick(&mut game_state, &mut metro, update);
	}
}