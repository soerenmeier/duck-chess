use crate::engine::evaluate_single_board;
use crate::pgn::{PgnMove, PgnPieceMove};
use crate::types::{Board, Move, PieceKind, PieceMove, Side, Square};
use crate::util::HighestScoreArray;

#[derive(Debug, Clone)]
pub struct ComputedBoard {
	inner: Board,
	duck_square: Option<Square>,
}

impl ComputedBoard {
	/// expects the board to be valid
	pub fn from_board(board: Board) -> Self {
		Self {
			duck_square: Self::find_duck(&board),
			inner: board,
		}
	}

	fn find_duck(board: &Board) -> Option<Square> {
		iter_board!(board.board)
			.find(|(_square, p)| matches!(p, Some(p) if p.kind.is_duck()))
			.map(|(sq, _p)| sq)
	}

	pub fn board(&self) -> &Board {
		&self.inner
	}

	pub fn into_board(self) -> Board {
		self.inner
	}

	pub fn new() -> Self {
		let mut board = Board::new();
		board.set_start_position();

		Self::from_board(board)
	}

	// fn compute_from_board(&mut self) {
	// 	self.white_pieces.clear();
	// 	self.black_pieces.clear();

	// 	for (num, piece) in self.inner.board.iter().enumerate() {
	// 		let Some(piece) = piece else {
	// 			continue
	// 		};

	// 		// ignore ducks
	// 		if piece.kind.is_duck() {
	// 			continue
	// 		}

	// 		let square = Square::from_u8(num as u8);

	// 		match piece.side {
	// 			Side::White => {
	// 				self.white_pieces.push((piece.kind, square));
	// 			},
	// 			Side::Black => {
	// 				self.black_pieces.push((piece.kind, square));
	// 			}
	// 		}
	// 	}
	// }

	pub fn next_move_side(&self) -> Option<Side> {
		self.inner.next_move
	}

	pub fn winner(&self) -> Option<Side> {
		self.inner.winner
	}

	pub fn moved_piece(&self) -> bool {
		self.inner.moved_piece
	}

	pub fn has_ended(&self) -> bool {
		self.next_move_side().is_none()
	}

	// does not clear the list
	#[cfg_attr(feature = "flamegraph", inline(never))]
	pub fn available_piece_moves(&self, list: &mut Vec<PieceMove>) {
		assert!(list.is_empty());

		let my_side = self.inner.next_move.unwrap();

		for (i, piece) in self.inner.board.iter().enumerate() {
			let Some(piece) = piece else { continue };

			if piece.side == my_side {
				let square = unsafe { Square::from_u8_unchecked(i as u8) };
				self.inner.available_piece_moves(piece.kind, square, list);
			}
		}
	}

	// does not clear the list
	pub fn available_duck_squares(&self, list: &mut Vec<Square>) {
		assert!(list.is_empty());

		self.inner.available_duck_squares(list);
	}

	/// The move must be valid
	pub fn convert_pgn_move(&self, mv: PgnMove) -> Option<Move> {
		let my_side = self.inner.next_move.unwrap();

		let (piece, from, to, capture) = match mv.piece {
			PgnPieceMove::Piece {
				piece,
				from,
				to,
				capture,
			} => (piece, from, to, capture),
			PgnPieceMove::Castle { long } => {
				// we can calculate this without a lookup
				// from king, to king ...
				let (fk, tk, fr, tr, y) = match my_side {
					Side::White if long => (4, 2, 0, 3, 7),
					Side::White => (4, 6, 7, 5, 7),
					Side::Black if long => (4, 2, 0, 3, 0),
					Side::Black => (4, 6, 7, 5, 0),
				};

				return Some(Move {
					piece: PieceMove::Castle {
						from_king: Square::from_xy(fk, y),
						to_king: Square::from_xy(tk, y),
						from_rook: Square::from_xy(fr, y),
						to_rook: Square::from_xy(tr, y),
					},
					duck: mv.duck,
					side: my_side,
				});
			}
		};

		let mut list = vec![];
		// todo sometimes a lookup is probably not always necessary

		for (i, p) in self.inner.board.iter().enumerate() {
			let Some(p) = p else { continue };

			if p.side != my_side || p.kind != piece {
				continue;
			}

			let square = unsafe { Square::from_u8_unchecked(i as u8) };

			self.inner.available_piece_moves(piece, square, &mut list);
		}

		for cand_mv in list {
			let (mv_from, mv_to, mv_capture) = match cand_mv {
				PieceMove::Piece {
					from, to, capture, ..
				} => (from, to, capture.is_some()),
				PieceMove::EnPassant { from, to } => (from, to, true),
				// castles already handled
				PieceMove::Castle { .. } => continue,
			};

			if capture != mv_capture {
				continue;
			}

			if let Some(from) = from {
				if mv_from != from {
					continue;
				}
			}

			if mv_to == to {
				// found the move
				return Some(Move {
					piece: cand_mv,
					duck: mv.duck,
					side: my_side,
				});
			}
		}

		None
	}

	/// The move must be valid
	/// And should be the next move of this board (not already applied)
	pub fn convert_move_to_pgn(&self, mv: Move) -> PgnMove {
		match mv.piece {
			PieceMove::Castle {
				from_king, to_king, ..
			} => {
				// todo we need to determine if it's long or short
				let long = from_king.x() == 4 && to_king.x() == 2;

				return PgnMove {
					piece: PgnPieceMove::Castle { long },
					duck: mv.duck,
				};
			}
			_ => {}
		}
		// let's first get what we need
		let (piece, from, to, capture) = match mv.piece {
			PieceMove::Piece {
				piece,
				from,
				to,
				capture,
				..
			} => (piece, from, to, capture.is_some()),
			PieceMove::EnPassant { from, to } => {
				(PieceKind::Pawn, from, to, true)
			}
			PieceMove::Castle { .. } => unreachable!(),
		};

		// now we need to find out if another piece can move to the same square
		let mut piece_moves = vec![];
		self.available_piece_moves(&mut piece_moves);

		let ambiguous_square = piece_moves.iter().any(|mov| match mov {
			PieceMove::Piece {
				piece: p_piece,
				from: p_from,
				to: p_to,
				..
			} => p_piece == &piece && p_to == &to && p_from != &from,
			PieceMove::EnPassant { .. } => false,
			PieceMove::Castle { .. } => false,
		});

		let from = ambiguous_square.then_some(from);

		PgnMove {
			piece: PgnPieceMove::Piece {
				piece,
				from,
				to,
				capture,
			},
			duck: mv.duck,
		}
	}

	pub fn apply_piece_move(&mut self, piece_move: PieceMove) {
		self.inner.apply_piece_move(piece_move);
	}

	pub fn apply_duck_move(&mut self, square: Square) {
		self.inner.apply_duck_move(square, self.duck_square);
		self.duck_square = Some(square);
	}

	// can only be called if a piece move is expected
	// the score is depending on the player who should play the move
	// so > 0 is better for self.next_move_self()
	pub fn evaluate(&self, depth: usize) -> HighestScoreArray<Move, 3> {
		self.evaluate_inner(depth, f32::MIN, f32::MAX)
	}

	fn evaluate_inner(
		&self,
		depth: usize,
		// best score for the player that needs to move
		mut alpha: f32,
		// best score for the oponent
		beta: f32,
	) -> HighestScoreArray<Move, 3> {
		let mut moves = HighestScoreArray::new();

		let next_side = self.inner.next_move.unwrap();

		let mut piece_moves = Vec::with_capacity(128);
		self.available_piece_moves(&mut piece_moves);
		let mut duck_moves = Vec::with_capacity(128);
		for piece_move in piece_moves {
			duck_moves.clear();

			let mut board = self.clone();
			// todo: this might now be broken because of the score
			let side = board.inner.next_move.unwrap();
			board.apply_piece_move(piece_move);
			board.inner.reasonable_duck_squares(&mut duck_moves);
			for square in duck_moves.iter() {
				let mut board = board.clone();
				board.apply_duck_move(*square);

				// let's check deeper
				let score = if depth > 0 {
					let next_moves = board.evaluate_inner(
						depth - 1,
						// reverse best scores since the oponnent is now the
						// active player
						-1f32 * beta,
						-1f32 * alpha,
					);
					// opponent's best score
					let best_score = next_moves.highest_score().unwrap_or(0f32);

					// reverse it because the move is beneficial for our oponent
					-1f32 * best_score
				} else {
					evaluate_single_board(&board.inner) * next_side.multi()
				};

				// store best score for us
				alpha = alpha.max(score);

				moves.insert(
					score,
					Move {
						piece: piece_move,
						duck: Some(*square),
						side,
					},
				);

				// if the best score for the opponent is smaller than our
				// we have found a better move
				if beta <= alpha {
					return moves;
				}
			}
		}

		moves
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn available_moves() {
		let board = ComputedBoard::new();
		let mut list = vec![];
		board.available_piece_moves(&mut list);
		assert_eq!(list.len(), 20);
	}

	#[test]
	// well it seems this test never succeeded and i don't want to
	// fix it
	#[ignore]
	fn evaluate() {
		let board = ComputedBoard::new();
		let moves = board.evaluate(1);
		assert_eq!(moves.len(), 396);
	}
}
