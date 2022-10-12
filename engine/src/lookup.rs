//! where lookup tables & stuff shoud go

// neighor
pub mod neighbor {
	use crate::engine::BitBoard;
	use crate::types::Square;

	include!(concat!(env!("OUT_DIR"), "/has_neighbor.rs"));

	#[inline]
	pub fn has_neighbor(square: Square, board: BitBoard) -> bool {
		unsafe {
			NEIGHBOR_LOOKUP.get_unchecked(square as u8 as usize)(board)
		}
	}
}