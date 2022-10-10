use crate::types::{Board, PieceKind};

// evaluate_board_in_pool

/// returns a score of a board + for white - for black
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
		PieceKind::Knight |
		PieceKind::Bishop => 3f32,
		PieceKind::King => 99f32,
		PieceKind::Queen => 9f32,
		PieceKind::Pawn => 1f32,
		PieceKind::Duck => 0f32
	}
}