//! Client implementation of the Metro Engine for WASM.
//! This code has all been written by-hand, with love.

use alloc::{boxed::Box, rc::Rc};

mod wasm {
	#[link(wasm_import_module = "metroSys")]
	unsafe extern "C" {
		/// The time (in milliseconds) since program start.
		#[link_name = "getTime"]
		pub unsafe fn sys_get_time() -> f64;

		/// Creates a handle to model data.
		#[link_name = "createModel"]
		pub unsafe fn sys_create_model(
			// *mut [[f32; 2]]
			positions_ptr: u32,
			// usize
			positions_len: u32,
			// *mut [[f32; 2]]
			coordinates_ptr: u32,
			// usize
			coordinates_len: u32,
			// *mut [[u16; 3]]
			indices_ptr: u32,
			// usize
			indices_len: u32,
		) -> u32; // Option<NonZeroU32>

		// Frees a previously created handle to model data.
		#[link_name = "dropModel"]
		pub unsafe fn sys_drop_model(model: u32) -> u32; // bool

		/// Creates a handle to a texture.
		#[link_name = "createTexture"]
		pub unsafe fn sys_create_texture(
			// *mut str
			url_ptr: u32,
			// usize
			url_len: u32,
		) -> u32; // Option<NonZeroU32>

		/// Frees a previously created handle to a texture.
		#[link_name = "dropTexture"]
		pub unsafe fn sys_drop_texture(texture: u32) -> u32; // bool

		/// Creates a handle to a shader.
		#[link_name = "createShader"]
		pub unsafe fn sys_create_shader(
			shader_stage: u32,
			source_ptr: u32,
			source_len: u32
		) -> u32; // Option<NonZeroU32>

		/// Frees a previously created handle to a shader.
		#[link_name = "dropShader"]
		pub unsafe fn sys_drop_shader(shader: u32) -> u32; // bool

		/// Creates a handle to a shader.
		#[link_name = "createMaterial"]
		pub unsafe fn sys_create_material(
			frag: u32,
			vert: u32
		) -> u32; // Option<NonZeroU32>

		/// Frees a previously created handle to a shader.
		#[link_name = "dropMaterial"]
		pub unsafe fn sys_drop_material(material: u32) -> u32; // bool
	}
}

pub struct ModelSys {
	handle: u32,
	is_fresh: bool,
}

impl ModelSys {
	pub fn create_model(
		positions: &Box<[[f32; 2]]>,
		coordinates: &Box<[[f32; 2]]>,
		indices: &Box<[[u16; 3]]>,
	) -> Rc<Self> {
		unsafe { 
			let handle = wasm::sys_create_model(
				positions.as_ptr() as u32,
				(positions.len() * 2) as u32,
				coordinates.as_ptr() as u32,
				(coordinates.len() * 2) as u32,
				indices.as_ptr() as u32,
				(indices.len() * 3) as u32,
			);
			if handle == 0 {
				panic!("Failed to create model in host-land, check the console for more info");
			}
			Rc::new(Self {
				handle,
				is_fresh: true,
			})
		}
	}
}

impl Drop for ModelSys {
	fn drop(&mut self) {
		if self.is_fresh {
			unsafe { wasm::sys_drop_model(self.handle); }
		}
	}
}

pub struct MeshSys {
	handle: u32,
	is_fresh: bool,
}

impl MeshSys {
	pub fn create_mesh(
		model: Rc<ModelSys>,
		material: Rc<MaterialSys>,
	) -> Rc<Self> {
	}
}

impl Drop for MeshSys {
	fn drop(&mut self) {
		if self.is_fresh {
			unsafe { wasm::sys_drop_model(self.handle); }
		}
	}
}

pub struct LightSys(u32);

pub struct TextureSys {
	handle: u32,
	is_fresh: bool,
}

impl TextureSys {
	pub fn create_texture(url: &str) -> Self {
		unsafe {
			let handle = wasm::sys_create_texture(
				url.as_ptr() as u32,
				url.len() as u32,
			);
			Self {
				handle,
				is_fresh: true,
			}
		}
	}
}

impl Drop for TextureSys {
	fn drop(&mut self) {
		unsafe {
			wasm::sys_drop_texture(self.handle);
		}
	}
}


// WebGL system:
// 1. represent the current state of rendering as a structure, on the client side.
// 2. lighting w/ 

#[cfg(all(not(test), target_arch = "wasm32"))]
#[panic_handler]
fn wasm_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
	// panic_info
	// TODO: stub, take logging facade impl from acrylic
	core::arch::wasm32::unreachable();
}