//! Client implementation of the Metra Engine for WASM.
//! This code has all been written by-hand, with love.

use core::num::NonZeroU32;

#[unsafe(export_name = "metraVarBigEndian")]
pub static mut METRA_HOST_BIG_ENDIAN: u32 = 0;

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

#[repr(u32)]
#[allow(unused)]
pub enum LogLevel {
	/// Designates very serious errors.
	Error = 1,
	/// Designates hazardous situations.
	Warn,
	/// Designates useful information.
	Info,
	/// Designates lower priority information.
	Debug,
	/// Designates very low priority, often extremely verbose, information.
	Trace,
	/// Used for panics
	Panic,
}

impl From<log::Level> for LogLevel {
	fn from(value: log::Level) -> Self {
		use log::Level::*;
		match value {
			Error => Self::Error,
			Warn => Self::Warn,
			Info => Self::Info,
			Debug => Self::Debug,
			Trace => Self::Trace,
		}
	}
}

#[link(wasm_import_module = "metraSys")]
unsafe extern "C" {
	#[link_name = "getRandom"]
	pub fn sys_get_random() -> f64;

	/// The time (in milliseconds) since program start.
	#[link_name = "getTime"]
	pub fn sys_get_time() -> f64;

	#[link_name = "log"]
	pub fn sys_log(
		level: LogLevel,
		target_ptr: u32,
		target_len: u32,
		location_ptr: u32,
		location_len: u32,
		content_ptr: u32,
		content_len: u32,
	);

	#[link_name = "createVertexArray"]
	pub fn create_vertex_array() -> Option<NonZeroU32>;
	#[link_name = "dropVertexArray"]
	pub fn drop_vertex_array(vertex_array: NonZeroU32) -> bool;

	#[link_name = "createBuffer"]
	pub fn create_buffer(
		buffer_type: BufferType,
		data_ptr: u32,
		data_len: u32,
	) -> Option<NonZeroU32>;
	#[link_name = "dropBuffer"]
	pub fn drop_buffer(buffer: NonZeroU32) -> bool;

	#[link_name = "createShader"]
	pub fn create_shader(stage: u32, source_ptr: u32, source_len: u32) -> Option<NonZeroU32>;
	#[link_name = "dropShader"]
	pub fn drop_shader(shader: NonZeroU32) -> bool;

	#[link_name = "createProgram"]
	pub fn create_program(vertex: NonZeroU32, fragment: NonZeroU32) -> Option<NonZeroU32>;
	#[link_name = "dropProgram"]
	pub fn drop_program(program: NonZeroU32) -> bool;

	#[link_name = "savePersistent"]
	pub fn save_persistent(data_ptr: u32, data_len: u32) -> bool;
	#[link_name = "loadPersistent"]
	pub fn load_persistent(data_ptr: u32, data_len: u32) -> u32;
}

#[cfg(all(not(test), target_arch = "wasm32"))]
#[panic_handler]
fn wasm_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
	use alloc::format;
	let message = format!("{}", panic_info.message());
	let loc = panic_info.location().map(|loc| format!("{loc}"));
	unsafe {
		sys_log(
			LogLevel::Panic,
			0,
			0,
			loc.as_ref().map(|s| s.as_ptr() as u32).unwrap_or(0),
			loc.as_ref().map(|s| s.len() as u32).unwrap_or(0),
			message.as_ptr() as u32,
			message.len() as u32,
		);
	}

	drop(message);
	drop(loc);
	core::arch::wasm32::unreachable();
}
