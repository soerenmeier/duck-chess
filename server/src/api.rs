use crate::error::Error;

use fire_api::request::{Request, Method};

use engine::types::{Board, PieceMove, Square, Side};

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
#[serde(rename_all = "camelCase")]
pub enum ApplyMove {
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