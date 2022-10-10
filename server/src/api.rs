use crate::error::Error;

use fire_api::request::{Request, Method};

use engine::types::{Board, PieceMove, Square, Side, Move};

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBoardReq;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBoard {
	pub board: Board
}

impl Request for NewBoardReq {
	type Response = NewBoard;
	type Error = Error;

	const PATH: &'static str = "/api/new-board";
	const METHOD: Method = Method::Get;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableMovesReq {
	pub board: Board
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AvailableMoves {
	Piece {
		moves: Vec<PieceMove>,
		side: Side
	},
	Duck {
		squares: Vec<Square>,
		side: Side
	}
}

impl Request for AvailableMovesReq {
	type Response = AvailableMoves;
	type Error = Error;

	const PATH: &'static str = "/api/available-moves";
	const METHOD: Method = Method::Post;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplyMoveReq {
	Piece {
		board: Board,
		mov: PieceMove
	},
	Duck {
		board: Board,
		mov: Square
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyMove {
	pub board: Board
}

impl Request for ApplyMoveReq {
	type Response = ApplyMove;
	type Error = Error;

	const PATH: &'static str = "/api/apply-move";
	const METHOD: Method = Method::Post;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateBoardReq {
	pub board: Board,
	pub depth: usize
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateBoard {
	// first one is the score
	pub moves: Vec<(f32, Move)>
}

impl Request for EvaluateBoardReq {
	type Response = EvaluateBoard;
	type Error = Error;

	const PATH: &'static str = "/api/evaluate-board";
	const METHOD: Method = Method::Post;
}