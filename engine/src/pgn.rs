use crate::types::{Piece, PieceKind, Square};

use byte_parser::{StrParser, ParseIterator};


#[derive(Debug, Clone)]
pub enum Error {
	NumberExpected,
	MismatchNumber,
	IncorrectMove,
	ExpectedSquare,
	UnknownPiece(u8)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgnMove {
	pub piece: PgnPieceMove,
	pub duck: Square
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PgnPieceMove {
	Piece {
		piece: PieceKind,
		from: Option<Square>,
		to: Square,
		capture: bool
	},
	Castle {
		long: bool
	}
}

pub fn parse_moves(s: &str) -> Result<Vec<PgnMove>, Error> {
	let mut parser = StrParser::new(s.trim());
	let mut moves = vec![];

	'main_loop: loop {
		parser.consume_while_byte_fn(u8::is_ascii_whitespace);

		if parser.peek().is_none() {
			break
		}

		let _ = parse_number(&mut parser)?;

		for _ in 0..2 {

			parser.consume_while_byte_fn(u8::is_ascii_whitespace);

			if let Some(mov) = parse_move(&mut parser)? {
				moves.push(mov);
			} else {
				break 'main_loop
			}
		}
	}

	Ok(moves)
}

fn parse_number<'a, I>(iter: &mut I) -> Result<usize, Error>
where I: ParseIterator<'a> {
	let mut iter = iter.record();

	// consume digits
	iter.while_byte_fn(u8::is_ascii_digit)
		.consume_at_least(1)
		.map_err(|_| Error::NumberExpected)?;

	let digits = iter.to_str();

	iter.expect_byte(b'.')
		.map_err(|_| Error::NumberExpected)?;

	digits.parse()
		.map_err(|_| Error::NumberExpected)
}

/// possible
/// O-O-O O-O hg1 Nf1 Bf1 Rhg1 R5a3 fxg3
fn parse_move<'a, I>(iter: &mut I) -> Result<Option<PgnMove>, Error>
where I: ParseIterator<'a> {
	let move_str = iter.record()
		.consume_while_byte_fn(|b| b.is_ascii_alphanumeric() || *b == b'-')
		.to_str();

	if matches!(move_str, "0-0" | "1-0" | "0-1") {
		return Ok(None)
	}

	let bytes = move_str.as_bytes();
	if bytes.len() < 4 {
		return Err(Error::IncorrectMove)
	}
	let (mut bytes, duck_bytes) = bytes.split_at(bytes.len() - 2);
	let duck = parse_square(duck_bytes[0], duck_bytes[1])?;

	if bytes == b"O-O" {
		return Ok(Some(PgnMove {
			piece: PgnPieceMove::Castle { long: false },
			duck
		}))
	} else if bytes == b"O-O-O" {
		return Ok(Some(PgnMove {
			piece: PgnPieceMove::Castle { long: true },
			duck
		}))
	}

	let mut piece = PieceKind::Pawn;
	if bytes[0].is_ascii_uppercase() {
		piece = match bytes[0] {
			b'R' => PieceKind::Rook,
			b'N' => PieceKind::Knight,
			b'B' => PieceKind::Bishop,
			b'K' => PieceKind::King,
			b'Q' => PieceKind::Queen,
			l => return Err(Error::UnknownPiece(l))
		};

		bytes = &bytes[1..];
	}

	let (mut bytes, square_bytes) = bytes.split_at(bytes.len() - 2);
	let to = parse_square(square_bytes[0], square_bytes[1])?;

	let capture = bytes.ends_with(&[b'x']);
	if capture {
		bytes = &bytes[..bytes.len() - 1];
	}

	let mut from = None;
	if bytes.len() == 1 {
		let byte = bytes[0];
		if byte.is_ascii_digit() {
			from = Some(parse_square(square_bytes[0], byte)?);
		} else if byte.is_ascii_alphabetic() && byte.is_ascii_lowercase() {
			from = Some(parse_square(byte, square_bytes[1])?);
		} else {
			return Err(Error::IncorrectMove)
		}
	} else if !bytes.is_empty() {
		return Err(Error::IncorrectMove)
	}

	Ok(Some(PgnMove {
		piece: PgnPieceMove::Piece { piece, from, to, capture },
		duck
	}))
}

fn parse_square(letter: u8, number: u8) -> Result<Square, Error> {
	assert_ne!(number, b'0');

	let number = number - b'1';

	let letter_number = letter - b'a';
	if letter_number >= 8 || number >= 8 {
		return Err(Error::ExpectedSquare)
	}

	// reverse number since the chess coordinates system has zero at the bottom
	let number = 7 - number;

	Ok(Square::from_xy(letter_number, number))
}


#[cfg(test)]
mod tests {
	use super::*;

	/// O-O-O O-O hg1 Nf1 Bf1 Rhg1 R5a3 fxg3
	#[test]
	fn test_parsing() {
		let moves = parse_moves("
			1. e2e4 d7d6
			2. Nb1c3 g7g6
			3. Ng1f3 Bf8g7
			4. Bf1e2 c7c6
		").unwrap();

		let correct_moves = vec![
			PgnMove {
				piece: PgnPieceMove::Piece {
					piece: PieceKind::Pawn,
					from: None,
					to: Square::E2,
					capture: false
				},
				duck: Square::E4
			},
			PgnMove {
				piece: PgnPieceMove::Piece {
					piece: PieceKind::Pawn,
					from: None,
					to: Square::D7,
					capture: false
				},
				duck: Square::D6
			},
			PgnMove {
				piece: PgnPieceMove::Piece {
					piece: PieceKind::Knight,
					from: None,
					to: Square::B1,
					capture: false
				},
				duck: Square::C3
			}
		];

		assert_eq!(&moves[..correct_moves.len()], &correct_moves);
	}
}