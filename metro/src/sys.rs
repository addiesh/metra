//! Client implementation of the Metro Engine for WASM.
//! This code has all been written by-hand, with love.

use alloc::{boxed::Box, rc::Rc};

use crate::ShaderStage;

#[unsafe(export_name = "metroVarBigEndian")]
static mut METRO_HOST_BIG_ENDIAN: u32 = 0;

mod wasm {
	#[repr(u32)]
	pub enum BufferType {
		/// Buffer containing vertex attributes, such as vertex coordinates,
		/// texture coordinate data, or vertex color data.
		Array = 0,
		/// Buffer used for element indices.
		Element = 1,
		/// Buffer used for storing uniform blocks.
		Uniform = 2,
	}

	#[link(wasm_import_module = "metroSys")]
	unsafe extern "C" {
		/// The time (in milliseconds) since program start.
		#[link_name = "getTime"]
		pub unsafe fn sys_get_time() -> f64;

		#[link_name = "createBuffer"]
		pub unsafe fn sys_create_buffer(
			bufferType: BufferType,
			dataPtr: u32,
			dataLen: u32,
		) -> u32;
	}
}

pub struct ModelSys {
	handle: u32,
	
}

impl ModelSys {
	pub fn create_model(
		positions: &Box<[[f32; 2]]>,
		coordinates: &Box<[[f32; 2]]>,
		indices: &Box<[[u16; 3]]>,
	) -> Rc<Self> {
		todo!()
	}
}

impl Drop for ModelSys {
	fn drop(&mut self) {
		todo!()
	}
}

pub struct ShaderSys {
	stage: ShaderStage,
	handle: u32,
}

impl ShaderSys {
	pub fn create_shader(stage: ShaderStage, source: &str) -> Rc<Self> {
		todo!()
	}
}

impl Drop for ShaderSys {
	fn drop(&mut self) {
		todo!()
	}
}

pub struct MaterialSys {
	handle: u32,
}

impl MaterialSys {
	pub fn create_material(
		vert: Rc<ShaderSys>,
		frag: Rc<ShaderSys>,
	) -> Rc<Self> {
		todo!()
	}
}

impl Drop for MaterialSys {
	fn drop(&mut self) {
		todo!()
	}
}

pub struct MeshSys {
	handle: u32,
	model: Rc<ModelSys>,
	material: Rc<MaterialSys>,
}

impl MeshSys {
	pub fn create_mesh(
		model: Rc<ModelSys>,
		material: Rc<MaterialSys>,
	) -> Self {
		todo!()
	}
}

impl Drop for MeshSys {
	fn drop(&mut self) {
		todo!()
	}
}

pub struct LightSys(u32);

pub struct TextureSys {
	handle: u32,
	
}

impl TextureSys {
	pub fn create_texture(url: &str) -> Self {
		todo!()
	}
}

impl Drop for TextureSys {
	fn drop(&mut self) {
		todo!()
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