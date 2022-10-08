
import Data from 'fire/data/data.js';
import Api from 'fire/api/api.js';
import { Board, PieceMove } from './data.js';

const api = new Api('http://127.0.0.1:1658/api');

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