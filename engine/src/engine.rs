use crate::types::{Board, PieceKind, Square};

// evaluate_board_in_pool

// we have threads that take work
// when putting the value into the pool you get a receipt (a token)

/// returns a score of a board + for white - for black
#[cfg_attr(feature = "flamegraph", inline(never))]
pub fn evaluate_single_board(board: &Board) -> f32 {
	// for the moment just count points
	let mut total = 0f32;

	for piece in board.board.iter() {
		if let Some(piece) = piece {
			total += piece.side.multi() * piece_kind_to_point(piece.kind);
		}
	}

	total
}

fn piece_kind_to_point(piece: PieceKind) -> f32 {
	match piece {
		PieceKind::Rook => 5f32,
		PieceKind::Knight | PieceKind::Bishop => 3f32,
		PieceKind::King => 99f32,
		PieceKind::Queen => 9f32,
		PieceKind::Pawn => 1f32,
		PieceKind::Duck => 0f32,
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard {
	board: u64,
}

impl BitBoard {
	pub fn new() -> Self {
		Self { board: 0 }
	}

	#[inline]
	pub fn set(&mut self, square: Square) {
		self.board |= 1 << square as u8
	}

	#[inline]
	pub fn set_val(&mut self, square: Square, val: bool) {
		self.board |= (val as u64) << square as u8
	}

	#[inline]
	pub const fn is_set(&self, square: Square) -> bool {
		self.board & (1 << square as u8) > 0
	}

	#[inline]
	pub const fn invert(&self) -> Self {
		Self { board: !self.board }
	}
}
