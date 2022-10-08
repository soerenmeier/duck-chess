
use std::mem;

#[derive(Debug, Clone)]
pub struct Board {
	pub board: [Option<Piece>; 64],
	// white, black
	pub can_castle: CanCastle,
	// if the last move was a pawn with two pushes store that position here
	pub en_passant: Option<Square>,
	pub next_move: Side
}

// Board
macro_rules! setup_board {
	($board:expr, $($id:tt),*) => (
		$board.board = [
			$(
				setup_board!($id)
			),*
		];
	);
	(e) => (None);
	(bR) => (Some(Piece { kind: PieceKind::Rook, side: Side::Black }));
	(bN) => (Some(Piece { kind: PieceKind::Knight, side: Side::Black }));
	(bB) => (Some(Piece { kind: PieceKind::Bishop, side: Side::Black }));
	(bQ) => (Some(Piece { kind: PieceKind::Queen, side: Side::Black }));
	(bK) => (Some(Piece { kind: PieceKind::King, side: Side::Black }));
	(bP) => (Some(Piece { kind: PieceKind::Pawn, side: Side::Black }));
	(wR) => (Some(Piece { kind: PieceKind::Rook, side: Side::White }));
	(wN) => (Some(Piece { kind: PieceKind::Knight, side: Side::White }));
	(wB) => (Some(Piece { kind: PieceKind::Bishop, side: Side::White }));
	(wQ) => (Some(Piece { kind: PieceKind::Queen, side: Side::White }));
	(wK) => (Some(Piece { kind: PieceKind::King, side: Side::White }));
	(wP) => (Some(Piece { kind: PieceKind::Pawn, side: Side::White }));
}

impl Board {
	pub fn new() -> Self {
		Self {
			board: [None; 64],
			can_castle: CanCastle { white: (true, true), black: (true, true) },
			en_passant: None,
			next_move: Side::White
		}
	}

	pub fn set_start_position(&mut self) {
		setup_board!(self,
			bR, bN, bB, bQ, bK, bB, bN, bR,
			bP, bP, bP, bP, bP, bP, bP, bP,
			e, e, e, e, e, e, e, e,
			e, e, e, e, e, e, e, e,
			e, e, e, e, e, e, e, e,
			e, e, e, e, e, e, e, e,
			wP, wP, wP, wP, wP, wP, wP, wP,
			wR, wN, wB, wQ, wK, wB, wN, wR
		);
		self.can_castle = CanCastle {
			white: (true, true),
			black: (true, true)
		};
		self.en_passant = None;
		self.next_move = Side::White;
	}

	pub fn piece_at(&self, square: Square) -> Option<Piece> {
		unsafe { *self.board.get_unchecked(square as u8 as usize) }
	}

	fn can_eat_piece(piece: Piece, side: Side) -> bool {
		!piece.kind.is_duck() && piece.side != side
	}

	/// returns (valid, captures)
	fn valid_square_for_piece(
		&self,
		square: Square,
		side: Side
	) -> (bool, Option<PieceKind>) {
		let Some(piece) = self.piece_at(square) else {
			return (true, None)
		};

		let valid = Self::can_eat_piece(piece, side);
		if valid {
			(true, Some(piece.kind))
		} else {
			(false, None)
		}
	}

	fn available_moves_by_dir(
		&self,
		from: Square,
		dirs: &[Direction],
		// if only one move can be done (true for king)
		max_one: bool,
		list: &mut Vec<PieceMove>
	) {
		let dist = if max_one { 1 } else { 8 };
		let Some(piece) = self.piece_at(from) else {
			panic!("no piece")
		};

		for dir in dirs {
			let mut to = from;
			for dist in 0..dist {
				if !to.apply_dir(*dir) {
					break
				}

				let (valid, capture) = self.valid_square_for_piece(
					to,
					piece.side
				);

				if !valid {
					break
				}

				let mv_kind = MoveKind::Piece {
					piece: piece.kind,
					from, to,
					capture,
					promotion: None
				};

				list.push(PieceMove { kind: mv_kind, side: piece.side });

				// cannot capture a piece after a capture
				if capture.is_some() {
					break
				}
			}
		}
	}

	// the square needs to be the position of the king
	pub fn available_castle_moves(
		&self,
		square: Square,
		list: &mut Vec<PieceMove>
	) {
		const LONG_FREE: &[u8] = &[1, 2, 3];
		const SHORT_FREE: &[u8] = &[5, 6];

		let Some(piece) = self.piece_at(square) else {
			panic!("no piece")
		};

		let (y, (can_castle_long, can_castle_short)) = match piece.side {
			Side::White => (7, self.can_castle.white),
			Side::Black => (0, self.can_castle.black)
		};

		if can_castle_long {
			let all_free = LONG_FREE.iter()
				.map(|x| Square::from_xy(*x, y))
				.all(|square| self.piece_at(square).is_none());

			if all_free {
				list.push(PieceMove {
					kind: MoveKind::Castle {
						from_king: square,
						to_king: Square::from_xy(2, y),
						from_rook: Square::from_xy(0, y),
						to_rook: Square::from_xy(3, y)
					},
					side: piece.side
				});
			}
		}

		if can_castle_short {
			let all_free = SHORT_FREE.iter()
				.map(|x| Square::from_xy(*x, y))
				.all(|square| self.piece_at(square).is_none());

			if all_free {
				list.push(PieceMove {
					kind: MoveKind::Castle {
						from_king: square,
						to_king: Square::from_xy(6, y),
						from_rook: Square::from_xy(7, y),
						to_rook: Square::from_xy(5, y)
					},
					side: piece.side
				});
			}
		}
	}

	pub fn available_pawn_moves(&self, square: Square, list: &mut Vec<PieceMove>) {
		const CAN_PROMOTE_TO: &[PieceKind] = &[
			PieceKind::Rook,
			PieceKind::Knight,
			PieceKind::Bishop,
			PieceKind::Queen
		];

		let Some(piece) = self.piece_at(square) else {
			panic!("no piece")
		};

		let (second_rank, to_promotion) = match piece.side {
			Side::White => (6, 1),
			Side::Black => (1, 6)
		};
		let can_promote = square.y() == to_promotion;

		let move_dist = if square.y() == second_rank {
			2
		} else {
			1
		};

		// move up
		{
			let mut up_square = square;
			for _ in 0..move_dist {
				if !up_square.apply_dir(Direction::Up) ||
					self.piece_at(up_square).is_some()
				{
					break
				}

				list.push(PieceMove {
					kind: MoveKind::Piece {
						piece: piece.kind,
						from: square,
						to: up_square,
						capture: None,
						promotion: None
					},
					side: piece.side
				});
			}
		}

		// promotion
		if can_promote {
			let promotion_square = square.add_dir(Direction::Up).unwrap();
			if self.piece_at(promotion_square).is_none() {
				for promotion_piece in CAN_PROMOTE_TO {
					list.push(PieceMove {
						kind: MoveKind::Piece {
							piece: piece.kind,
							from: square,
							to: promotion_square,
							capture: None,
							promotion: Some(*promotion_piece)
						},
						side: piece.side
					});
				}
			}
		}

		// take piece
		for dir in &[Direction::UpLeft, Direction::UpRight] {
			let Some(new_square) = square.add_dir(*dir) else {
				continue
			};

			let Some(eat_piece) = self.piece_at(new_square) else {
				continue
			};

			if Self::can_eat_piece(eat_piece, piece.side) {
				list.push(PieceMove {
					kind: MoveKind::Piece {
						piece: piece.kind,
						from: square,
						to: new_square,
						capture: Some(eat_piece.kind),
						promotion: None
					},
					side: piece.side
				});
			}
		}

		// en passant
		if let Some(en_passant_square) = self.en_passant {
			// needs to be besides the pawn and on the same rank
			if en_passant_square.y() != square.y() ||
				square.x().abs_diff(en_passant_square.x()) != 1
			{
				return
			}

			let new_square = Square::from_xy(
				en_passant_square.x(),
				// since en passant can only happen after a two distance move
				// we know that the pawn will endup on the second rank
				second_rank
			);

			list.push(PieceMove {
				kind: MoveKind::EnPassant {
					from: square,
					to: new_square
				},
				side: piece.side
			});
		}
	}

	pub fn available_knight_moves(
		&self,
		square: Square,
		list: &mut Vec<PieceMove>
	) {
		let Some(piece) = self.piece_at(square) else {
			panic!("no piece")
		};

		for dir in Direction::ALL_KNIGHTS {
			let Some(new_square) = square.add_dir(*dir) else {
				continue
			};

			let capture = if let Some(eat_piece) = self.piece_at(new_square) {
				if !Self::can_eat_piece(eat_piece, piece.side) {
					continue
				}

				Some(eat_piece.kind)
			} else {
				None
			};

			list.push(PieceMove {
				kind: MoveKind::Piece {
					piece: piece.kind,
					from: square,
					to: new_square,
					capture,
					promotion: None
				},
				side: piece.side
			});
		}
	}

	pub fn available_piece_moves(
		&self,
		piece: PieceKind,
		square: Square,
		list: &mut Vec<PieceMove>
	) {
		match piece {
			PieceKind::Rook |
			PieceKind::Bishop |
			PieceKind::Queen => {
				self.available_moves_by_dir(
					square,
					piece.directions(),
					false,
					list
				);
			},
			PieceKind::King => {
				self.available_moves_by_dir(
					square,
					piece.directions(),
					true,
					list
				);

				self.available_castle_moves(square, list);
			},
			PieceKind::Pawn => {
				self.available_pawn_moves(square, list);
			},
			PieceKind::Knight => {
				self.available_knight_moves(square, list);
			},
			PieceKind::Duck => unreachable!()
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
	pub kind: PieceKind,
	pub side: Side
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
	Rook,
	Knight,
	Bishop,
	King,
	Queen,
	Pawn,
	Duck
}

impl PieceKind {
	// returns an empty slice if the piece does not work with directions
	pub fn directions(&self) -> &'static [Direction] {
		use Direction::*;
		match self {
			Self::Rook => &[Up, Right, Down, Left],
			Self::Knight => &[],
			Self::Bishop => &[UpRight, DownRight, DownLeft, UpLeft],
			Self::King => Direction::ALL_NO_KNIGHTS,
			Self::Queen => Direction::ALL_NO_KNIGHTS,
			Self::Pawn => &[],
			Self::Duck => &[]
		}
	}

	pub fn is_duck(&self) -> bool {
		matches!(self, Self::Duck)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanCastle {
	// long, short
	pub white: (bool, bool),
	pub black: (bool, bool)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
	White,
	Black
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
	pub piece: PieceMove,
	pub duck: Square
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PieceMove {
	pub kind: MoveKind,
	pub side: Side
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveKind {
	Piece {
		piece: PieceKind,
		from: Square,
		to: Square,
		capture: Option<PieceKind>,
		promotion: Option<PieceKind>
	},
	EnPassant {
		from: Square,
		to: Square
	},
	Castle {
		from_king: Square,
		to_king: Square,
		from_rook: Square,
		to_rook: Square
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Square {
    A8 = 0, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

impl Square {
	// ## Panics if the number is 64 >=
	#[inline]
	pub fn from_u8(num: u8) -> Self {
		assert!(num < 64);
		unsafe { mem::transmute(num) }
	}

	pub fn x(&self) -> u8 {
		*self as u8 % 8
	}

	pub fn y(&self) -> u8 {
		*self as u8 / 8
	}

	// needs to be valid coordinates 0-7 0-7
	#[inline]
	pub fn from_xy(x: u8, y: u8) -> Self {
		Square::from_u8(y * 8 + x)
	}

	#[inline]
	pub fn add_dir(&self, dir: Direction) -> Option<Self> {
		let mut xy = (self.x() as i8, self.y() as i8);
		dir.update_xy(&mut xy);

		let (x, y) = xy;
		if x < 0 || x >= 8 || y < 0 || y >= 8 {
			return None
		}

		Some(Self::from_xy(x as u8, y as u8))
	}

	/// returns true if the move was applied
	#[inline]
	pub fn apply_dir(&mut self, dir: Direction) -> bool {
		if let Some(next) = self.add_dir(dir) {
			*self = next;
			true
		} else {
			false
		}
	}


}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
	Up,
	UpRight,
	Right,
	DownRight,
	Down,
	DownLeft,
	Left,
	UpLeft,
	KnUpRight,
	KnRightUp,
	KnRightDown,
	KnDownRight,
	KnDownLeft,
	KnLeftDown,
	KnLeftUp,
	KnUpLeft
}

impl Direction {
	const ALL_NO_KNIGHTS: &[Self] = &[
		Self::Up,
		Self::UpRight,
		Self::Right,
		Self::DownRight,
		Self::Down,
		Self::DownLeft,
		Self::Left,
		Self::UpLeft
	];

	const ALL_KNIGHTS: &[Self] = &[
		Self::KnUpRight,
		Self::KnRightUp,
		Self::KnRightDown,
		Self::KnDownRight,
		Self::KnDownLeft,
		Self::KnLeftDown,
		Self::KnLeftUp,
		Self::KnUpLeft
	];

	fn xy_change(&self) -> (i8, i8) {
		match self {
			Self::Up => (0, -1),
			Self::UpRight => (1, -1),
			Self::Right => (1, 0),
			Self::DownRight => (1, 1),
			Self::Down => (0, 1),
			Self::DownLeft => (-1, 1),
			Self::Left => (-1, 1),
			Self::UpLeft => (-1, -1),
			// knight
			Self::KnUpRight => (1, -2),
			Self::KnRightUp => (2, -1),
			Self::KnRightDown => (2, 1),
			Self::KnDownRight => (1, 2),
			Self::KnDownLeft => (-1, 2),
			Self::KnLeftDown => (-2, 1),
			Self::KnLeftUp => (-2, -1),
			Self::KnUpLeft => (-1, -2),
		}
	}

	fn update_xy(&self, xy: &mut (i8, i8)) {
		let change = self.xy_change();
		xy.0 += change.0;
		xy.1 += change.1;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn sq_xy(square: Square) -> (u8, u8) {
		(square.x(), square.y())
	}

	#[test]
	fn square_xy() {
		assert_eq!(sq_xy(Square::A8), (0, 0));
		assert_eq!(sq_xy(Square::H8), (7, 0));
		assert_eq!(sq_xy(Square::A1), (0, 7));
		assert_eq!(sq_xy(Square::H1), (7, 7));
		assert_eq!(Square::from_xy(0, 0), Square::A8);
		assert_eq!(Square::from_xy(7, 0), Square::H8);
		assert_eq!(Square::from_xy(0, 7), Square::A1);
		assert_eq!(Square::from_xy(7, 7), Square::H1);
	}

	#[test]
	fn apply_dir() {
		let mut square = Square::E4;
		assert!(square.apply_dir(Direction::Up));
		assert_eq!(square, Square::E5);
		assert!(square.apply_dir(Direction::UpLeft));
		assert_eq!(square, Square::D6);
	}
}