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

#[derive(Copy, Clone)]
#[repr(packed)]
struct UniversalShaderUniform {
	time: f32,
	transform: [f32; 4],
}

impl Default for UniversalShaderUniform {
	fn default() -> Self {
		Self {
			time: 0.0,
			transform: [1.0, 0.0, 0.0, 1.0],
		}
	}
}

impl UniversalShaderUniform {
	const _ASSERT: () = [()][size_of::<UniversalShaderUniform>() - 20];
}

pub struct Metra {
	meshes: Vec<NonNull<ResourceTarget<Mesh>>>,
	lights: Vec<NonNull<ResourceTarget<Light>>>,
	effects: Vec<NonNull<ResourceTarget<Effect>>>,
	shaders: Vec<NonNull<Material>>,

	unit_quad: Asset<Model>,
	universal_vertex: NonZeroU32,
	default_material: Asset<Material>,
	// standard_vertex:
	// model_cache
}

static UNIT_QUAD: ([[f32; 2]; 4], [[f32; 2]; 4], [[u16; 3]; 2]) = (
	[[-1., -1.], [1.0, -1.], [1.0, 1.0], [-1., 1.0]],
	[[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
	[[0, 1, 2], [0, 2, 3]],
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

		let universal_vertex = unsafe {
			sys::create_shader(
				0,
				UNIVERSAL_VERTEX_SHADER.as_ptr() as u32,
				UNIVERSAL_VERTEX_SHADER.len() as u32,
			)
			.expect("Failed to compile fragment shader!")
		};

		let default_material = Asset::new(Material::new(DEFAULT_FRAGMENT_SHADER, universal_vertex));

		Self {
			meshes,
			lights,
			shaders,
			effects,
			unit_quad,
			universal_vertex,
			default_material,
		}
	}

	#[inline]
	pub(crate) fn update_internal(&mut self, status: MetraStatus) {
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

		// self.universal_shader_uniform
		// 	.upload(&[UniversalShaderUniform {
		// 		time: unsafe { sys::get_time() as f32 },
		// 		transform: [1.0, 0.0, 0.0, 1.0],
		// 	}]);

		// TODO: use uniform blocks to share universal shader inputs (time/transform)!!!
		//		 you already added support for uniform buffers anyway
		iterate_targets(&mut self.meshes, |mesh| {
			// TODO: render mesh
			unsafe {
				sys::bind_vertex_array(mesh.model.vertex_array_handle);
				mesh.model.positions.bind();
				mesh.model.coordinates.bind();
				mesh.model.indices.bind();
				mesh.material.bind();
				// the multiplication is because each triangle is an element in the greater array
				let index_count = mesh.model.indices.len() as u32 * 3;
				sys::draw_triangles(index_count);
				debug!("drew mesh with {index_count} indices");
			}
		});
		// TODO: render everything!
	}

	/// Returns the time elapsed since the start of the game, in milliseconds.
	#[must_use]
	#[inline]
	pub fn time(&self) -> f64 {
		unsafe { sys::get_time() }
	}

	/// Returns a pseudo-random number from 0 to 1.
	#[must_use]
	#[inline]
	pub fn random(&self) -> f64 {
		unsafe { sys::get_random() }
	}

	#[must_use]
	#[inline]
	pub fn unit_quad(&self) -> Asset<Model> {
		self.unit_quad.clone()
	}

	#[must_use]
	#[inline]
	pub fn new_unit_mesh(&mut self) -> Resource<Mesh> {
		self.new_mesh(self.unit_quad.clone(), self.default_material.clone())
	}

	#[must_use]
	pub fn new_mesh(&mut self, model: Asset<Model>, material: Asset<Material>) -> Resource<Mesh> {
		let (resource, ptr) = Resource::new(Mesh::new(model, material.clone()));
		self.meshes.push(ptr);
		resource
	}

	#[must_use]
	pub fn new_light(&mut self, x: f32, y: f32) -> Resource<Light> {
		let (resource, ptr) = Resource::new(Light { x, y });
		self.lights.push(ptr);
		resource
	}

	/// Writes a buffer to the persistent data store.
	/// Returns 1 on success, or 0 otherwise.
	#[must_use]
	pub fn save_persistent(&mut self, data: &[u8]) -> bool {
		unsafe {
			let decoded = base64::engine::general_purpose::STANDARD.encode(data);
			let res = sys::save_persistent(decoded.as_ptr() as u32, decoded.len() as u32);
			drop(decoded);
			res
		}
	}

	/// Returns
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

struct Buffer<T: Copy> {
	data: Option<Box<[T]>>,
	handle: NonZeroU32,
	buffer_type: sys::BufferType,
}

impl<T: Copy> Buffer<T> {
	fn new(buffer_type: sys::BufferType, data: Option<&[T]>) -> Self {
		unsafe {
			let handle = sys::create_buffer();

			let data: Option<Box<[T]>> = data.map(Box::from);

			if let Some(data) = &data {
				sys::bind_buffer(handle, buffer_type);
				sys::upload_buffer_data(
					buffer_type,
					data.as_ptr() as u32,
					size_of_val(data.deref()) as u32,
				);
			}

			let this = Self {
				data,
				handle,
				buffer_type,
			};

			this
		}
	}

	fn bind(&self) {
		unsafe {
			sys::bind_buffer(self.handle, self.buffer_type);
		}
	}

	fn upload(&mut self, data: &[T]) {
		unsafe {
			self.bind();
			let data: Box<[T]> = Box::from(data);
			sys::upload_buffer_data(
				self.buffer_type,
				data.as_ptr() as u32,
				size_of_val(data.deref()) as u32,
			);
			self.data = Some(data);
		}
	}

	fn len(&self) -> usize {
		self.data.as_ref().map_or(0, |b| b.len())
	}

	/// Returns the size, in bytes, to the buffer.
	fn size(&self) -> usize {
		self.data.as_ref().map_or(0, |b| size_of_val(b.deref()))
	}
}

impl<T: Copy> Drop for Buffer<T> {
	fn drop(&mut self) {
		unsafe {
			sys::drop_buffer(self.handle);
		}
	}
}

pub struct Model {
	positions: Buffer<[f32; 2]>,
	coordinates: Buffer<[f32; 2]>,
	indices: Buffer<[u16; 3]>,
	vertex_array_handle: NonZeroU32,
}

impl Model {
	fn new(positions: &[[f32; 2]], coordinates: &[[f32; 2]], indices: &[[u16; 3]]) -> Self {
		unsafe {
			let vao = sys::create_vertex_array();
			sys::bind_vertex_array(vao);

			debug!("creating positions");
			// creating a buffer also binds it. this is a quirk. just ignore it.
			let positions = Buffer::new(sys::BufferType::Array, Some(positions));
			positions.bind();
			sys::enable_vertex_attrib(0);
			sys::vertex_attrib_pointer(0, 2, 0, false);

			debug!("creating coordinates");
			let coordinates = Buffer::new(sys::BufferType::Array, Some(coordinates));
			coordinates.bind();
			sys::enable_vertex_attrib(1);
			sys::vertex_attrib_pointer(1, 2, 0, false);

			debug!("creating indices");
			let indices = Buffer::new(sys::BufferType::Element, Some(indices));

			Self {
				positions,
				coordinates,
				indices,
				vertex_array_handle: vao,
			}
		}
	}
}

impl Drop for Model {
	fn drop(&mut self) {
		unsafe {
			debug!("dropping model");
			sys::drop_vertex_array(self.vertex_array_handle);
		}
	}
}

/// Meshes

pub struct Mesh {
	model: Asset<Model>,
	material: Asset<Material>,
}

impl Mesh {
	fn new(model: Asset<Model>, material: Asset<Material>) -> Self {
		Self { model, material }
	}
}

impl Drop for Mesh {
	fn drop(&mut self) {
		debug!("dropping mesh");
	}
}

pub struct Light {
	x: f32,
	y: f32,
}

pub struct Sound {}

pub struct Texture {
	width: u32,
	height: u32,
}

pub struct Material {
	program: NonZeroU32,
}

impl Material {
	fn new(fragment_source: &'static str, vertex: NonZeroU32) -> Self {
		unsafe {
			let fragment = sys::create_shader(
				1,
				fragment_source.as_ptr() as u32,
				fragment_source.len() as u32,
			)
			.expect("Failed to compile fragment shader!");
			let program =
				sys::create_program(vertex, fragment).expect("Failed to create shader program!");
			sys::drop_shader(fragment);
			Self { program }
		}
	}

	fn bind(&self) {
		unsafe {
			sys::bind_program(self.program);
		}
	}
}

impl Drop for Material {
	fn drop(&mut self) {
		debug!("dropping material");
		unsafe {
			sys::drop_program(self.program);
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
	{ $manifest:expr, $init:expr, $update:expr } => {
		#[unsafe(export_name = "metraMain")]
		extern "C" fn __metra_main() {
			::metra::run(
				$manifest,
				$init,
				$update
			)
		}
	};
}

pub struct MetraResourceManifest;

/// This function initializes the Metra engine,
/// and should only be called from within the [metra_main!] macro.
// can you tell we're committing crimes against Rust?
// We **have** to use global state here in order to get the registration-based API I really wanted.
//
#[allow(static_mut_refs)]
#[inline]
pub fn run<T: 'static>(
	_manifest: MetraResourceManifest,
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
						engine.update_internal(status);
					} else {
						engine.update_internal(status);
					}

					status
				}
			}
		}));
	}
}
