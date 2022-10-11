//! where lookup tables & stuff shoud go

// neighor
pub mod neighbor {
	use crate::types::{Square, Board};

	include!(concat!(env!("OUT_DIR"), "/has_neighbor.rs"));

	#[inline]
	pub fn has_neighbor(square: Square, board: &Board) -> bool {
		unsafe {
			NEIGHBOR_LOOKUP.get_unchecked(square as u8 as usize)(board)
		}
	}
}