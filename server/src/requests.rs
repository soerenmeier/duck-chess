
use crate::Data;
use crate::error::Error;
use crate::api::{
	NewBoardReq, NewBoard,
	AvailableMovesReq, AvailableMoves,
	ApplyMoveReq, ApplyMove
};

use engine::types::{Board};
use engine::logic::ComputedBoard;

use fire::FireBuilder;
use fire_api::request_handler;


request_handler! {
	async fn new_board(_req: NewBoardReq) -> Result<NewBoard, Error> {
		let mut board = Board::new();
		board.set_start_position();

		Ok(NewBoard { board })
	}
}

request_handler! {
	async fn available_moves(
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
}

request_handler! {
	async fn apply_move(req: ApplyMoveReq) -> Result<ApplyMove, Error> {
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
}

pub fn add_routes(server: &mut FireBuilder<Data>) {
	server.add_route(new_board);
	server.add_route(available_moves);
	server.add_route(apply_move);
}