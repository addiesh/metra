#![no_std]
extern crate alloc;

#[global_allocator]
static DLMALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use core::ops::{Deref, DerefMut};

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;

mod arena;
pub mod prelude;
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
	meshes: Vec<ResourceTarget<Mesh>>,
	lights: Vec<ResourceTarget<Light>>,
	effects: Vec<ResourceTarget<Effect>>,
	// standard_vertex:
	// model_cache
}

static UNIT_QUAD: ([f32; 8], [f32; 8], [u16; 6]) = (
	[-1., -1., 1.0, -1., 1.0, 1.0, -1., 1.0],
	[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
	[0, 1, 2, 2, 3, 0],
);

impl Metro {
	fn new() -> Self {
		Self {
			meshes: Vec::with_capacity(128),
			lights: Vec::with_capacity(32),
			effects: Vec::with_capacity(4),
		}
	}

	/// Returns the time elapsed since the start of the game, in milliseconds.
	#[must_use]
	#[inline]
	pub fn time(&self) -> f64 {
		unsafe { sys::sys_get_time() }
	}

	/// Returns a psuedo-random number from 0 to 1.
	#[must_use]
	#[inline]
	pub fn random(&self) -> f64 {
		unsafe { sys::sys_get_random() }
	}

	#[must_use]
	pub fn new_mesh(&mut self, model: Asset<Model>, material: Asset<Material>) -> Resource<Mesh> {
		Resource::new(Mesh { model, material })
	}

	#[must_use]
	pub fn new_light(&mut self, source: LightSource) -> Light {
		Light { source }
	}

	#[inline]
	#[must_use]
	pub fn save_persistent(&mut self, data: &[u8]) -> bool {
		sys::save_persistent(data)
	}

	#[inline]
	#[must_use]
	pub fn load_persistent(&self) -> Option<Box<[u8]>> {
		sys::load_persistent()
	}

	// pub fn get_action_state<T>(&self) -> { }
	// pub fn did_just_action<T>(&self) -> bool {}
	// pub fn load_asset(&self) -> Asset {}
}

/// The borrow checker is able to enforce most of the rules
/// regarding lifetimes in the engine, but the most problematic
/// exception is the smart pointer type.
struct OwnershipFailsafe {}

pub struct Resource<T> {
	target: *mut ResourceTarget<T>,
}

// SAFETY: resource will always be alive while the handle itself is alive.
impl<T> Deref for Resource<T> {
	type Target = T;

	fn deref<'a>(&'a self) -> &'a Self::Target {
		unsafe { &self.target.as_ref::<'a>().unwrap_unchecked().data }
	}
}

impl<T> DerefMut for Resource<T> {
	fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
		unsafe { &mut self.target.as_mut::<'a>().unwrap_unchecked().data }
	}
}

struct ResourceTarget<T> {
	in_use: bool,
	data: T,
}

impl<T> Resource<T> {
	fn new(data: T) -> Self {
		Self {
			target: Box::leak(Box::new(ResourceTarget { in_use: true, data })),
		}
	}
}

impl<T> Drop for Resource<T> {
	fn drop(&mut self) {
		unsafe { drop(Box::from_raw(self.target)) }
	}
}

pub struct Asset<T> {
	rc: Rc<T>,
}

pub struct Model {
	positions: Box<[[f32; 2]]>,
	coordinates: Box<[[f32; 2]]>,
	indices: Box<[[u16; 3]]>,
	from_static: bool,
}
pub struct Mesh {
	model: Asset<Model>,
	material: Asset<Material>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LightSource {
	Point,
	Ambient,
}
pub struct Light {
	source: LightSource,
}
pub struct Sound {}
pub struct Texture {
	width: u32,
	height: u32,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ShaderStage {
	Vertex = 0,
	Fragment = 1,
}
pub struct Shader {}
pub struct Material {}
pub struct Effect {}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MetroStatus {
	/// Stops execution whenever possible and releases assets and resources.
	Stop = 0,
	/// Continues execution as normal.
	Continue = 1,
	// /// Continues execution, but with a higher tolerance for delays.
	// Yield = 2,
}

/// The beginning of a Metro game.
#[macro_export]
macro_rules! metro_main {
	{ $init:expr, $update:expr } => {
		#[unsafe(export_name = "metroMain")]
		extern "C" fn __generated_metro_main() {
			::metro::run(
				$init,
				$update
			)
		}
	};
}

/// This function initializes the metro engine,
/// and should only be called from within the [metro_main!] macro.
// can you tell we're committing crimes against Rust?
#[allow(static_mut_refs)]
#[inline]
pub fn run<T: 'static>(
	init: fn(engine: &mut Metro) -> T,
	update: fn(state: &mut T, engine: &mut Metro) -> MetroStatus,
) {
	static mut HAS_SETUP: bool = false;
	// possibly replace with a cell in the future?
	static mut UPDATE_FN: Option<Box<dyn FnMut() -> MetroStatus>> = None;

	#[unsafe(export_name = "metroUpdate")]
	extern "C" fn _update() -> MetroStatus {
		unsafe { (UPDATE_FN.as_mut().unwrap())() }
	}

	#[unsafe(export_name = "metroClean")]
	extern "C" fn _clean() -> () {
		unsafe {
			drop(UPDATE_FN.take().unwrap());
		}
	}

	unsafe {
		if HAS_SETUP {
			panic!("run is not allowed to be called multiple times");
		} else {
			HAS_SETUP = true;
		}
	}

	sys::init();

	// 1. initialize the engine state
	// 2. initialize the game state, provide access to the engine handle to load assets
	// 3. run 1 game frame detached from the remaining game loop
	// 4. show the game window (if applicable)
	// 5. loop game frames until closing
	// 6. hide the game window and free resources

	// let mut metro = Rc::new(Metro::new());
	let mut engine = Metro::new();

	// TODO: initialize the engine state

	let mut game_state = init(&mut engine);

	// show window

	unsafe {
		UPDATE_FN = Some(Box::new(move || update(&mut game_state, &mut engine)));
	}
}
