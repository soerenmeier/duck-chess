use crate::types::{PieceKind, Square, Side, Board, Move};


pub struct ComputedBoard {
	inner: Board,
	// no ducks
	white_pieces: Vec<(PieceKind, Square)>,
	// no ducks
	black_pieces: Vec<(PieceKind, Square)>
}

impl ComputedBoard {
	pub fn new() -> Self {
		let mut board = Board::new();
		board.set_start_position();

		let mut this = Self {
			inner: board,
			white_pieces: vec![],
			black_pieces: vec![]
		};

		this.compute_from_board();

		this
	}

	fn compute_from_board(&mut self) {
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

	pub fn available_piece_moves(&self, list: &mut Vec<Move>) {
		let pieces = match self.inner.next_move {
			Side::White => &self.white_pieces,
			Side::Black => &self.black_pieces
		};

		for (piece, square) in pieces {
			self.inner.available_piece_moves(*piece, *square, list);
		}
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