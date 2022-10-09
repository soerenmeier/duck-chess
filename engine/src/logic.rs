use crate::types::{PieceKind, Square, Side, Board, Move, PieceMove};
use crate::pgn::{PgnMove, PgnPieceMove};

pub struct ComputedBoard {
	inner: Board,
	// no ducks
	white_pieces: Vec<(PieceKind, Square)>,
	// no ducks
	black_pieces: Vec<(PieceKind, Square)>
}

impl ComputedBoard {
	/// expects the board to be valid
	pub fn from_board(board: Board) -> Self {
		let mut this = Self {
			inner: board,
			white_pieces: vec![],
			black_pieces: vec![]
		};

		this.compute_from_board();

		this
	}

	pub fn into_board(self) -> Board {
		self.inner
	}

	pub fn new() -> Self {
		let mut board = Board::new();
		board.set_start_position();

		Self::from_board(board)
	}

	fn compute_from_board(&mut self) {
		self.white_pieces.clear();
		self.black_pieces.clear();

		for (num, piece) in self.inner.board.iter().enumerate() {
			let Some(piece) = piece else {
				continue
			};

			// ignore ducks
			if piece.kind.is_duck() {
				continue
			}

			let square = Square::from_u8(num as u8);

			match piece.side {
				Side::White => {
					self.white_pieces.push((piece.kind, square));
				},
				Side::Black => {
					self.black_pieces.push((piece.kind, square));
				}
			}
		}
	}

	pub fn next_move_side(&self) -> Side {
		self.inner.next_move
	}

	pub fn moved_piece(&self) -> bool {
		self.inner.moved_piece
	}

	pub fn available_piece_moves(&self, list: &mut Vec<PieceMove>) {
		let pieces = match self.inner.next_move {
			Side::White => &self.white_pieces,
			Side::Black => &self.black_pieces
		};

		for (piece, square) in pieces {
			self.inner.available_piece_moves(*piece, *square, list);
		}
	}

	pub fn available_duck_squares(&self, list: &mut Vec<Square>) {
		self.inner.available_duck_squares(list);
	}

	/// The move must be valid
	pub fn convert_pgn_move(&self, mv: PgnMove) -> Move {
		let (piece, from, to, capture) = match mv.piece {
			PgnPieceMove::Piece { piece, from, to, capture } => {
				(piece, from, to, capture)
			},
			PgnPieceMove::Castle { long } => {
				// we can calculate this without a lookup
				// from king, to king ...
				let (fk, tk, fr, tr, y) = match self.inner.next_move {
					Side::White if long => (4, 2, 0, 3, 7),
					Side::White => (4, 6, 7, 5, 7),
					Side::Black if long => (4, 2, 0, 3, 0),
					Side::Black => (4, 6, 7, 5, 0)
				};

				return Move {
					piece: PieceMove::Castle {
						from_king: Square::from_xy(fk, y),
						to_king: Square::from_xy(tk, y),
						from_rook: Square::from_xy(fr, y),
						to_rook: Square::from_xy(tr, y)
					},
					duck: mv.duck,
					side: self.inner.next_move
				}
			}
		};

		let mut list = vec![];
		// todo sometimes a lookup is probably not always necessary

		let pieces = match self.inner.next_move {
			Side::White => &self.white_pieces,
			Side::Black => &self.black_pieces
		};

		for (p, square) in pieces {
			if piece != *p {
				continue
			}

			self.inner.available_piece_moves(piece, *square, &mut list);
		}

		for cand_mv in list {
			let (mv_from, mv_to, mv_capture) = match cand_mv {
				PieceMove::Piece { from, to, capture, .. } => {
					(from, to, capture.is_some())
				},
				PieceMove::EnPassant { from, to } => (from, to, true),
				// castles already handled
				PieceMove::Castle { .. } => continue
			};

			if capture != mv_capture {
				continue
			}

			if let Some(from) = from {
				if mv_from != from {
					continue
				}
			}

			if mv_to == to {
				// found the move
				return Move {
					piece: cand_mv,
					duck: mv.duck,
					side: self.inner.next_move
				}
			}
		}

		panic!("no move found")
	}

	pub fn apply_piece_move(&mut self, piece_move: PieceMove) {
		self.inner.apply_piece_move(piece_move);
		// todo optimize this
		self.compute_from_board();
	}

	pub fn apply_duck_move(&mut self, square: Square) {
		self.inner.apply_duck_move(square);

		// since the duck is not stored in pieces we don't need to update it
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn available_moves() {
		let mut board = ComputedBoard::new();
		let mut list = vec![];
		board.available_piece_moves(&mut list);
		assert_eq!(list.len(), 20);
	}
}