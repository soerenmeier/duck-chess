use std::{env, fs};
use std::path::Path;
use std::fmt::Write;

use dunce::canonicalize;


fn main() {
	println!("cargo:rerun-if-changed=../ui/dist");

	let out_dir = env::var("OUT_DIR").unwrap();
	let assets_routes = Path::new(&out_dir).join("assets_routes.rs");

	let ui_dist = Path::new("../ui/dist");
	if !ui_dist.is_dir() {
		fs::write(assets_routes, "\
			use fire::FireBuilder;\n\
			pub fn add_routes(_: &mut FireBuilder) {}")
			.unwrap();
		return
	}

	let mut assets = vec![];

	let assets_dir = ui_dist.join("assets");
	for entry in fs::read_dir(&assets_dir).unwrap() {
		let entry = entry.unwrap();

		if entry.file_type().unwrap().is_dir() {
			panic!("dir not supported {:?}", entry);
		}

		let name = entry.file_name().into_string().unwrap();
		assets.push((
			format!("/assets/{name}"),
			canonicalize(entry.path()).unwrap()
		));
	}


	let mut s = String::new();
	write!(s, "use fire::{{memory_file, fs::MemoryFile, FireBuilder}};\n\n")
		.unwrap();

	// add index
	let index_path = canonicalize("../ui/dist/index.html").unwrap();
	write!(s, "const INDEX: MemoryFile = \
		memory_file!(\"/\", {index_path:?});\n").unwrap();

	for (i, (uri, path)) in assets.iter().enumerate() {
		write!(s, "const ASSET_{i}: MemoryFile = \
			memory_file!({uri:?}, {path:?});\n").unwrap();
	}

	write!(s, "\npub fn add_routes(fire: &mut FireBuilder) {{\n").unwrap();

	// add index
	write!(s, "\tfire.add_route(INDEX);\n").unwrap();

	for (i, _) in assets.iter().enumerate() {
		write!(s, "\tfire.add_route(ASSET_{i});\n").unwrap();
	}

	write!(s, "}}\n").unwrap();

	
	fs::write(assets_routes, s).unwrap();
}