use crate::sys::sys_log;
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::ToString;
use log::Log;

struct MetraLogger;
const LOG_LEVEL_FILTER: log::LevelFilter = log::LevelFilter::Trace;
static LOGGER: MetraLogger = MetraLogger;

impl Log for MetraLogger {
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

pub fn init() {
	log::set_max_level(LOG_LEVEL_FILTER);
	log::set_logger(&LOGGER).unwrap();
}
