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
			"const fn has_neighbor_{i}(board: BitBoard) -> bool {{\n"
		).unwrap();

		let mut exprs = vec![];

		for (d_x, d_y) in NEIGHBOR_MATRICES {
			let x = x + d_x;
			let y = y + d_y;

			if x < 0 || x >= 8 || y < 0 || y >= 8 {
				continue
			}

			let idx = x + y * 8;

			exprs.push(format!(
				"\tboard.is_set(Square::from_u8({idx})) "
			));
		}

		let exprs: String = exprs.join("||\n");
		write!(s, "{exprs}\n}}\n\n").unwrap();
	}

	// build lookup table
	write!(s, "const NEIGHBOR_LOOKUP: [fn(BitBoard) -> bool; 64] = [\n")
		.unwrap();
	for i in 0..64 {
		write!(s, "\thas_neighbor_{i},\n").unwrap();
	}
	write!(s, "];\n").unwrap();


	fs::write(path.as_ref().join("has_neighbor.rs"), s).unwrap();
}