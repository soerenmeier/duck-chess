use std::{env, fs};
use std::path::Path;
use std::fmt::Write;


fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();

	generate_has_neighbor(&out_dir);
}

const NEIGHBOR_MATRICES: [(i8, i8); 8] = [
	(-1, -1), (0, -1), (1, -1),
	(-1, 0), (1, 0),
	(-1, 1), (0, 1), (1, 1)
];

/// this is a bit overkill since it doesn't bring alot of performance increase
/// but since the code already exists, why not keep it?
fn generate_has_neighbor(path: impl AsRef<Path>) {
	// generate fns
	let mut s = String::new();

	for i in 0..64 {
		let x = i as i8 % 8;
		let y = i as i8 / 8;

		write!(s,
			"fn has_neighbor_{i}(board: &Board) -> bool {{\n"
		);

		let mut exprs = vec![];

		for (dX, dY) in NEIGHBOR_MATRICES {
			let x = x + dX;
			let y = y + dY;

			if x < 0 || x >= 8 || y < 0 || y >= 8 {
				continue
			}

			let idx = x + y * 8;

			exprs.push(format!(
				"\tboard.piece_at(Square::from_u8({idx})).is_some() "
			));
		}

		let exprs: String = exprs.join("||\n");
		write!(s, "{exprs}\n}}\n\n");
	}

	// build lookup table
	write!(s, "const NEIGHBOR_LOOKUP: [fn(&Board) -> bool; 64] = [\n");
	for i in 0..64 {
		write!(s, "\thas_neighbor_{i},\n");
	}
	write!(s, "];\n");


	fs::write(path.as_ref().join("has_neighbor.rs"), s);
}