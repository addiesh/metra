use std::env::current_dir;
use std::fs::{DirEntry, ReadDir};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs, io};

fn run_rustfmt(code: String) -> io::Result<String> {
	let rustfmt = if let Ok(rustfmt) = env::var("RUSTFMT") {
		rustfmt.into()
	} else {
		which::which("rustfmt").expect("Failed to find rustfmt using \"which\" !")
	};

	let mut cmd = Command::new(&*rustfmt);

	cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

	let mut child = cmd.spawn().expect("Failed to execute rustfmt!");
	let mut child_stdin = child.stdin.take().unwrap();
	let mut child_stdout = child.stdout.take().unwrap();

	// Write to stdin in a new thread, so that we can read from stdout on this
	// thread. This keeps the child from blocking on writing to its stdout which
	// might block us from writing to its stdin.
	let stdin_handle = ::std::thread::spawn(move || {
		let _ = child_stdin.write_all(code.as_bytes());
		code
	});

	let mut output = vec![];
	io::copy(&mut child_stdout, &mut output)?;

	let status = child.wait()?;
	let source = stdin_handle.join().expect(
		"The thread writing to rustfmt's stdin doesn't do \
             anything that could panic",
	);

	match String::from_utf8(output) {
		Ok(bindings) => match status.code() {
			Some(0) => Ok(bindings),
			Some(2) => Err(io::Error::new(
				io::ErrorKind::Other,
				"Rustfmt parsing errors.".to_string(),
			)),
			Some(3) => {
				eprintln!("Rustfmt could not format some lines.");
				Ok(bindings)
			}
			_ => Err(io::Error::new(
				io::ErrorKind::Other,
				"Internal rustfmt error".to_string(),
			)),
		},
		_ => Ok(source),
	}
}

fn main() {
	// let assets_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "assets"].iter().collect();
	let assets_path: PathBuf = current_dir().unwrap();
	let assets_dir = fs::read_dir(&assets_path).expect("Failed to find asset directory!");

	{
		let canonical = assets_path
			.canonicalize()
			.unwrap()
			.to_string_lossy()
			.to_string();
		// println!("cargo::rerun-if-changed={canonical}");
	}

	let mut path_stack: Vec<(ReadDir, Option<PathBuf>)> = vec![(assets_dir, None)];

	let mut out_string = String::new();
	// out_string += "use super::resource::*;";

	while !path_stack.is_empty() {
		let (read_dir, dir_path) = path_stack.pop().unwrap();
		let entries: Vec<DirEntry> = read_dir.into_iter().map(|de| de.unwrap()).collect();

		let category_name: Option<String> = dir_path.as_ref().map(|dir| {
			dir.file_name()
				.unwrap()
				.to_string_lossy()
				.to_lowercase()
				.chars()
				.filter_map(|ch| {
					if ch == '_' || ch.is_alphanumeric() {
						Some(ch)
					} else if ch == '-' || ch == '.' {
						Some('_')
					} else {
						None
					}
				})
				.collect()
		});

		if let Some(dir_path) = &dir_path {
			let canonical = dir_path
				.canonicalize()
				.unwrap()
				.to_string_lossy()
				.to_string();
			// println!("cargo::rerun-if-changed={canonical}");
		}

		// ignore directories with a .ignore_assets file
		if entries
			.iter()
			.find(|de| {
				let meta = de.metadata().unwrap();
				meta.is_file() && de.file_name() == ".ignore_assets"
			})
			.is_some()
		{
			continue;
		}

		if let Some(category_name) = &category_name {
			out_string += &format!("mod {category_name} {{");
		}

		// out_string
		for entry in entries {
			let meta = entry.metadata().unwrap();
			if meta.is_dir() {
				path_stack.push((entry.path().read_dir().unwrap(), Some(entry.path())));
				continue;
			}
			let name: String = entry
				.path()
				.file_stem()
				.unwrap()
				.to_string_lossy()
				.to_uppercase()
				.chars()
				.filter_map(|ch| {
					if ch.is_alphanumeric() {
						Some(ch)
					} else if ch == '_' || ch == '-' || ch == '.' {
						Some('_')
					} else {
						None
					}
				})
				.collect();
			let canonical: String = entry
				.path()
				.canonicalize()
				.unwrap()
				.to_string_lossy()
				.into();
			// println!("cargo::rerun-if-changed={canonical}");

			if fs::read_to_string(entry.path()).is_ok() {
				out_string += &format!("static {name}: &str = include_str!(\"{canonical}\");");
			} else {
				out_string += &format!("static {name}: &[u8] = include_bytes!(\"{canonical}\");");
			}
		}

		if category_name.is_some() {
			out_string += "}";
		}
	}

	out_string = run_rustfmt(out_string).expect("Failed to format generated code!");

	let gen_src_path: PathBuf = [&env::var("OUT_DIR").unwrap(), "assets.rs"]
		.iter()
		.collect();
	fs::write(gen_src_path, out_string).unwrap()
}
