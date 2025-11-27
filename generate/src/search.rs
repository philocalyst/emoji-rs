use std::{fs::File, io::{Read, Write}, path::PathBuf, process::Command};

pub fn dump() {
	let mut fs = String::new();
	let mut f = File::open("generate/src/search_header.rs").unwrap();
	f.read_to_string(&mut fs).unwrap();

	let path = "emoji/src/search.rs";
	let pb: PathBuf = path.into();
	File::create(pb).unwrap().write_all(fs.as_bytes()).unwrap();
	Command::new("rustfmt").arg(path).output().expect("Failed to execute command");
}
