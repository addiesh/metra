#![no_std]

extern crate alloc;

#[cfg(not(target_arch = "wasm32"))]
compile_error!("Metra must target WASM32");

// lol, lmao.
// #![forbid(unsafe_code)]

#[global_allocator]
static DLMALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use core::ops::Deref;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::{borrow::ToOwned, boxed::Box};
use base64::Engine;
use core::num::NonZeroU32;
use core::ptr::NonNull;
use log::{debug, error, warn};

use crate::{asset::Asset, prelude::Resource, resource::ResourceTarget};

pub mod asset;
mod logger;
pub mod prelude;
pub mod resource;
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

pub struct Metra {
	meshes: Vec<NonNull<ResourceTarget<Mesh>>>,
	lights: Vec<NonNull<ResourceTarget<Light>>>,
	effects: Vec<NonNull<ResourceTarget<Effect>>>,
	shaders: Vec<NonNull<Shader>>,

	unit_quad: Asset<Model>,
	// TODO: shader pretty explicitly means "fragment shader" here.
	//		 This is a blatant misuse of API.
	universal_vertex: Asset<Shader>,
	default_fragment: Asset<Shader>,
	// standard_vertex:
	// model_cache
}

static UNIT_QUAD: ([[f32; 2]; 4], [[f32; 2]; 4], [[u16; 3]; 2]) = (
	[[-1., -1.], [1.0, -1.], [1.0, 1.0], [-1., 1.0]],
	[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
	[[0, 1, 2], [2, 3, 0]],
);

static UNIVERSAL_VERTEX_SHADER: &str = include_str!("universal_vertex.glsl");
static DEFAULT_FRAGMENT_SHADER: &str = include_str!("default_fragment.glsl");

impl Metra {
	pub(crate) fn new() -> Self {
		let meshes = Vec::with_capacity(128);
		let lights = Vec::with_capacity(32);
		let shaders = Vec::with_capacity(16);
		let effects = Vec::with_capacity(4);

		let unit_quad = Asset::new(Model::new(&UNIT_QUAD.0, &UNIT_QUAD.1, &UNIT_QUAD.2));

		let universal_vertex = Asset::new(Shader::new(UNIVERSAL_VERTEX_SHADER, true));

		let default_fragment = Asset::new(Shader::new(DEFAULT_FRAGMENT_SHADER, false));

		Self {
			meshes,
			lights,
			shaders,
			effects,
			unit_quad,
			universal_vertex,
			default_fragment,
		}
	}

	#[inline]
	pub(crate) fn pre_update_internal(&mut self) {}

	#[inline]
	pub(crate) fn post_update_internal(&mut self, status: MetraStatus) {
		fn iterate_targets<T>(
			targets: &mut Vec<NonNull<ResourceTarget<T>>>,
			func: fn(target: &mut T),
		) {
			targets.retain_mut(|a| unsafe {
				// SAFETY: `a` is always allocated by a Box, ensuring proper alignment,
				//		It will only ever be deallocated by this function
				let target = a.as_mut();
				match &mut target.data {
					None => {
						// remove from array and release resources.
						// SAFETY: the NonNull will never be accessed again (removed from vec)
						//		by the end of this block.
						let boxed: Box<ResourceTarget<T>> = Box::from_raw(a.as_mut());
						drop(boxed);
						false
					}
					Some(data) => {
						// run the closure and keep in the array
						func(data);
						true
					}
				}
			});
		}
		iterate_targets(&mut self.meshes, |mesh| {
			// TODO: render mesh
		});
		// TODO: render everything!
	}

	/// Returns the time elapsed since the start of the game, in milliseconds.
	#[must_use]
	#[inline]
	pub fn time(&self) -> f64 {
		unsafe { sys::sys_get_time() }
	}

	/// Returns a pseudo-random number from 0 to 1.
	#[must_use]
	#[inline]
	pub fn random(&self) -> f64 {
		unsafe { sys::sys_get_random() }
	}

	pub fn unit_quad(&self) -> Asset<Model> {
		self.unit_quad.clone()
	}

	pub fn new_unit_mesh(&mut self) -> Resource<Mesh> {
		self.new_mesh(self.unit_quad.clone(), self.default_fragment.clone())
	}

	#[must_use]
	pub fn new_mesh(&mut self, model: Asset<Model>, shader: Asset<Shader>) -> Resource<Mesh> {
		let (resource, ptr) = Resource::new(Mesh::new(
			model,
			self.universal_vertex.clone(),
			shader.clone(),
		));
		self.meshes.push(ptr);
		resource
	}

	#[must_use]
	pub fn new_light(&mut self, source: LightSource) -> Light {
		Light { source }
	}

	#[must_use]
	pub fn save_persistent(&mut self, data: &[u8]) -> bool {
		unsafe {
			let decoded = base64::engine::general_purpose::STANDARD.encode(data);
			let res = sys::save_persistent(decoded.as_ptr() as u32, decoded.len() as u32);
			drop(decoded);
			res
		}
	}

	#[must_use]
	pub fn load_persistent(&self) -> Option<Box<[u8]>> {
		unsafe {
			match sys::load_persistent(0, 0) {
				u32::MAX => None,
				0 => Some(Box::new([])),
				size => {
					let mut allocation = Box::new_uninit_slice(size as usize);
					let res = sys::load_persistent(allocation.as_mut_ptr() as u32, size);
					if res == u32::MAX {
						panic!("Size of persistent data changed while performing load!");
					}
					let text = allocation.assume_init();
					let decoded = base64::engine::general_purpose::STANDARD.decode(text);
					match decoded {
						Ok(v) => Some(v.into_boxed_slice()),
						Err(err) => {
							error!("DecodeError during persistent data load, {err}");
							None
						}
					}
				}
			}
		}
	}

	// pub fn get_action_state<T>(&self) -> { }
	// pub fn did_just_action<T>(&self) -> bool {}
	// pub fn load_asset(&self) -> Asset {}
}

impl Drop for Metra {
	fn drop(&mut self) {
		debug!("engine state dropped");
	}
}

pub struct Buffer {}

pub struct Model {
	positions: Box<[[f32; 2]]>,
	coordinates: Box<[[f32; 2]]>,
	indices: Box<[[u16; 3]]>,
	// webGL specifics
	position_handle: NonZeroU32,
	coordinate_handle: NonZeroU32,
	index_handle: NonZeroU32,
	vertex_array_object: NonZeroU32,
}

impl Model {
	fn new(positions: &[[f32; 2]], coordinates: &[[f32; 2]], indices: &[[u16; 3]]) -> Self {
		unsafe {
			let vao = sys::create_vertex_array().expect("failed to create VAO");
			let positions: Box<[[f32; 2]]> = positions.into_iter().map(|e| *e).collect();
			let coordinates: Box<[[f32; 2]]> = coordinates.into_iter().map(|e| *e).collect();
			let indices: Box<[[u16; 3]]> = indices.into_iter().map(|e| *e).collect();
			let position_handle = sys::create_buffer(
				sys::BufferType::Array,
				positions.as_ptr() as u32,
				size_of_val(positions.deref()) as u32,
			)
			.expect("failed to create position buffer (RESOURCE LEAK)");

			let coordinate_handle = sys::create_buffer(
				sys::BufferType::Array,
				coordinates.as_ptr() as u32,
				size_of_val(coordinates.deref()) as u32,
			)
			.expect("failed to create coordinate buffer (RESOURCE LEAK)");

			let index_handle = sys::create_buffer(
				sys::BufferType::Element,
				indices.as_ptr() as u32,
				size_of_val(indices.deref()) as u32,
			)
			.expect("failed to create index buffer (RESOURCE LEAK)");

			// todo!()
			Self {
				positions,
				coordinates,
				indices,
				position_handle,
				coordinate_handle,
				index_handle,
				vertex_array_object: vao,
			}
		}
	}
}

impl Drop for Model {
	fn drop(&mut self) {
		unsafe {
			debug!("dropping model");
			if !sys::drop_buffer(self.position_handle) {
				error!("Failed to drop position buffer!")
			}
			if !sys::drop_buffer(self.coordinate_handle) {
				error!("Failed to drop coordinate buffer!")
			}
			if !sys::drop_buffer(self.index_handle) {
				error!("Failed to drop index buffer!")
			}
			if !sys::drop_vertex_array(self.vertex_array_object) {
				error!("Failed to drop vertex array!")
			}
		}
	}
}

/// Meshes

pub struct Mesh {
	model: Asset<Model>,
	shader: Asset<Shader>,
	// TODO: this makes re-using shader programs impossible. This is, of course, silly.
	// 		Fix with a shader program cache?
	program: NonZeroU32,
}

impl Mesh {
	fn new(model: Asset<Model>, vertex: Asset<Shader>, fragment: Asset<Shader>) -> Self {
		let program = unsafe {
			sys::create_program(vertex.shader, fragment.shader)
				.expect("Failed to create mesh shader program!")
		};
		Self {
			model,
			shader: fragment,
			program,
		}
	}
}

impl Drop for Mesh {
	fn drop(&mut self) {
		debug!("dropping mesh");
		if unsafe { !sys::drop_program(self.program) } {
			error!("Failed to drop mesh shader program!")
		}
	}
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

pub struct Shader {
	source: String,
	shader: NonZeroU32,
}

impl Shader {
	fn new(source: &str, is_vertex: bool) -> Self {
		let source = source.to_owned();
		let shader = unsafe {
			sys::create_shader(
				(!is_vertex) as u32,
				source.as_ptr() as u32,
				source.len() as u32,
			)
			.expect("Failed to compile shader!")
		};
		Self { source, shader }
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			debug!("dropping shader");
			if !sys::drop_shader(self.shader) {
				error!("Failed to drop shader!")
			}
		}
	}
}

pub struct Effect {}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MetraStatus {
	/// Stops execution whenever possible and releases assets and resources.
	Stop = 0,
	/// Continues execution as normal.
	Continue = 1,
	// /// Continues execution, but with a higher tolerance for delays.
	// /// Use this for things like pause menus, where performance is less of a concern.
	// Yield = 2,
}

/// The beginning of a Metra game.
#[macro_export]
macro_rules! metra_main {
	{ $init:expr, $update:expr } => {
		#[unsafe(export_name = "metraMain")]
		extern "C" fn __metra_main() {
			::metra::run(
				$init,
				$update
			)
		}
	};
}

/// This function initializes the Metra engine,
/// and should only be called from within the [metra_main!] macro.
// can you tell we're committing crimes against Rust?
// We **have** to use global state here in order to get the registration-based API I really wanted.
//
#[allow(static_mut_refs)]
#[inline]
pub fn run<T: 'static>(
	init_callback: fn(engine: &mut Metra) -> T,
	update_callback: fn(state: &mut T, engine: &mut Metra) -> MetraStatus,
) {
	static mut HAS_SETUP: bool = false;
	// possibly replace with a cell in the future?
	static mut UPDATE_FN: Option<Box<dyn FnMut() -> MetraStatus>> = None;

	#[unsafe(export_name = "metraUpdate")]
	extern "C" fn metra_update() -> MetraStatus {
		unsafe { UPDATE_FN.as_mut().unwrap()() }
	}

	#[unsafe(export_name = "metraClean")]
	extern "C" fn metra_clean() -> () {
		unsafe {
			debug!("dropping closure");
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

	logger::init();

	let mut engine = Metra::new();
	// The game state will be set to None when the game requests quit (via MetaStatus::Stop)
	let mut game_state = Some(init_callback(&mut engine));
	unsafe {
		UPDATE_FN = Some(Box::new(move || {
			// This closure captures the engine and the game state,
			// which at this point have known sizes.
			engine.pre_update_internal();

			match &mut game_state {
				None => {
					warn!("update called after game state was dropped");
					MetraStatus::Stop
				}
				Some(state) => {
					let status = update_callback(state, &mut engine);

					if status == MetraStatus::Stop {
						debug!("dropping game state");
						game_state = None;
						debug!("game state dropped, all assets/resources should be freed.");
						debug!("performing final internal update");
						engine.post_update_internal(status);
					} else {
						engine.post_update_internal(status);
					}

					status
				}
			}
		}));
	}
}
