use crate::error::Error;
use crate::api::{
	NewBoardReq, NewBoard,
	AvailableMovesReq, AvailableMoves,
	ApplyMoveReq, ApplyMove,
	EvaluateBoardReq, EvaluateBoard
};

use tokio::task::spawn_blocking;

use engine::types::{Board};
use engine::logic::ComputedBoard;

use fire::{api, FireBuilder};


#[api(NewBoardReq)]
fn new_board() -> Result<NewBoard, Error> {
	let mut board = Board::new();
	board.set_start_position();

	Ok(NewBoard { board })
}

#[api(AvailableMovesReq)]
fn available_moves(
	req: AvailableMovesReq
) -> Result<AvailableMoves, Error> {
	let board = ComputedBoard::from_board(req.board);

	if board.moved_piece() {
		let mut list = vec![];
		board.available_duck_squares(&mut list);
		Ok(AvailableMoves::Duck {
			squares: list,
			side: board.next_move_side()
		})
	} else {
		let mut list = vec![];
		board.available_piece_moves(&mut list);
		Ok(AvailableMoves::Piece {
			moves: list,
			side: board.next_move_side()
		})
	}
}

#[api(ApplyMoveReq)]
fn apply_move(req: ApplyMoveReq) -> Result<ApplyMove, Error> {
	match req {
		// piece move
		ApplyMoveReq::Piece { board, mov } => {
			let mut board = ComputedBoard::from_board(board);

			board.apply_piece_move(mov);

			Ok(ApplyMove { board: board.into_board() })
		},
		// duck move
		ApplyMoveReq::Duck { board, mov } => {
			let mut board = ComputedBoard::from_board(board);

			board.apply_duck_move(mov);

			Ok(ApplyMove { board: board.into_board() })
		}
	}
}

#[api(EvaluateBoardReq)]
async fn evaluate_board(
	req: EvaluateBoardReq
) -> Result<EvaluateBoard, Error> {
	let board = ComputedBoard::from_board(req.board);

	let moves = spawn_blocking(move || {
		board.evaluate(req.depth)
	}).await.unwrap();

	Ok(EvaluateBoard { moves: moves.to_vec() })
}

pub fn add_routes(server: &mut FireBuilder) {
	server.add_route(new_board);
	server.add_route(available_moves);
	server.add_route(apply_move);
	server.add_route(evaluate_board);
}