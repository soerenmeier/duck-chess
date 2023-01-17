import Data from 'fire/data/data.js';
import Api from 'fire/api/api.js';
import { Board, PieceMove, Move } from './data.js';

let addr = '/api';
if (import.meta.env.MODE !== 'production')
	addr = 'http://127.0.0.1:1658/api';
const api = new Api(addr);

export class NewBoard extends Data {
	constructor(d) {
		super({
			board: Board
		}, d);
	}
}

export async function newBoard() {
	const d = await api.request('GET', '/new-board');
	return (new NewBoard(d)).board;
}

const possibleMoves = ['Piece', 'Duck'];

export class AvailableMoves extends Data {
	constructor(d) {
		if (typeof d !== 'object')
			throw new Error('Expected Object');

		let kind = possibleMoves.find(k => k in d);
		if (!kind)
			throw new Error('Pice Move not found');
		d = d[kind];

		switch (kind) {
			case 'Piece':
				super({
					moves: [PieceMove],
					side: 'str'
				}, d);
				break;
			case 'Duck':
				super({
					squares: ['str'],
					side: 'str'
				}, d);
				break;
		}

		this.kind = kind;
	}
}

export async function availableMoves(board) {
	const d = await api.request('POST', '/available-moves', { board });
	return new AvailableMoves(d);
}

export class ApplyMove extends Data {
	constructor(d) {
		super({
			board: Board
		}, d);
	}
}

// kind: Piece | Duck, mov: PieceMove | Square
export async function applyMove(kind, mov, board) {
	const obj = {};
	obj[kind] = { board, mov };
	const d = await api.request('POST', '/apply-move', obj);
	return (new ApplyMove(d)).board;
}

export class EvaluateBoard extends Data {
	constructor(d) {
		super({
			moves: [['float', Move]]
		}, d);
	}
}

export async function evaluateBoard(board, depth) {
	const d = await api.request('POST', '/evaluate-board', { board, depth });
	return (new EvaluateBoard(d)).moves;
}