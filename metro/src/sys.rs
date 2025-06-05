//! Client implementation of the Metro Engine for WASM.
//! This code has all been written by-hand, with love.

use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use alloc::{borrow::ToOwned, string::String};
use base64::Engine;
use log::{Log, debug, error, trace};

#[unsafe(export_name = "metroVarBigEndian")]
static mut METRO_HOST_BIG_ENDIAN: u32 = 0;

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

#[link(wasm_import_module = "metroSys")]
unsafe extern "C" {
	#[link_name = "getRandom"]
	pub unsafe fn sys_get_random() -> f64;

	/// The time (in milliseconds) since program start.
	#[link_name = "getTime"]
	pub unsafe fn sys_get_time() -> f64;

	#[link_name = "log"]
	pub unsafe fn sys_log(
		level: LogLevel,
		target_ptr: u32,
		target_len: u32,
		location_ptr: u32,
		location_len: u32,
		content_ptr: u32,
		content_len: u32,
	);

	#[link_name = "createBuffer"]
	pub unsafe fn sys_create_buffer(bufferType: BufferType, dataPtr: u32, dataLen: u32) -> u32;

	#[link_name = "savePersistent"]
	pub unsafe fn sys_save_persistent(dataPtr: u32, dataLen: u32) -> u32;
	#[link_name = "loadPersistent"]
	pub unsafe fn sys_load_persistent(dataPtr: u32, dataLen: u32) -> u32;
}

struct MetroLogger;
const LOG_LEVEL_FILTER: log::LevelFilter = log::LevelFilter::Trace;
static LOGGER: MetroLogger = MetroLogger;

impl Log for MetroLogger {
	fn enabled(&self, metadata: &log::Metadata) -> bool {
		metadata.level() <= LOG_LEVEL_FILTER
	}

	fn log(&self, record: &log::Record) {
		let message = format!("{}", record.args());
		let file = record.file();
		let line = record.line();
		let location = format!(
			"{}{}{}",
			file.map(|s| s.to_owned()).unwrap_or_default(),
			if file.and(line).is_some() { ":" } else { "" },
			line.map(|n| n.to_string()).unwrap_or_default(),
		);
		unsafe {
			sys_log(
				record.level().into(),
				record.target().as_ptr() as u32,
				record.target().len() as u32,
				location.as_ptr() as u32,
				location.len() as u32,
				message.as_ptr() as u32,
				message.len() as u32,
			);
		}

		drop(message);
		drop(location);
	}

	// unused
	fn flush(&self) {}
}

pub fn save_persistent(data: &[u8]) -> bool {
	unsafe {
		let decoded = base64::engine::general_purpose::STANDARD.encode(data);
		let res = sys_save_persistent(decoded.as_ptr() as u32, decoded.len() as u32);
		drop(decoded);
		res == 1
	}
}

pub fn load_persistent() -> Option<Box<[u8]>> {
	unsafe {
		match sys_load_persistent(0, 0) {
			u32::MAX => None,
			0 => Some(Box::new([])),
			size => {
				let mut allocation = Box::new_uninit_slice(size as usize);
				let res = sys_load_persistent(allocation.as_mut_ptr() as u32, size);
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

/// Right now, this just calls the logging facade functions.
pub fn init() {
	log::set_max_level(LOG_LEVEL_FILTER);
	log::set_logger(&LOGGER).unwrap();
}

// WebGL system:
// 1. represent the current state of rendering as a structure, on the client side.
// 2. lighting w/

#[cfg(all(not(test), target_arch = "wasm32"))]
#[panic_handler]
fn wasm_panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
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
