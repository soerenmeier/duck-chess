import Data from 'fire/data/data.js';
import { Option } from 'fire/data/parsetypes.js';
import { range } from 'fire/util.js';

export class Board extends Data {
	constructor(d) {
		super({
			board: [new Option(Piece)],
			canCastle: CanCastle,
			enPassant: 'optstr',
			nextMove: 'str',
			movedPiece: 'bool'
		}, d);
	}

	static empty() {
		return new Board({
			board: range(0, 64).map(() => null),
			canCastle: { white: [true, true], black: [true, true] },
			enPassant: null,
			nextMove: 'White',
			movedPiece: false
		});
	}

	getPiece(i) {
		return this.board[i];
	}

	// returns the index or -1
	duckPosition() {
		return this.board.findIndex(p => p?.kind === 'Duck');
	}
}

export class Piece extends Data {
	constructor(d) {
		super({
			kind: 'str',
			side: 'str'
		}, d);
	}
}

export class CanCastle extends Data {
	constructor(d) {
		super({
			white: ['bool', 'bool'],
			black: ['bool', 'bool']
		}, d);
	}
}

const possiblePieceMoves = ['Piece', 'EnPassant', 'Castle'];

export class PieceMove extends Data {
	constructor(d) {
		if (typeof d !== 'object')
			throw new Error('Expected Object');

		let kind = possiblePieceMoves.find(k => k in d);
		if (!kind)
			throw new Error('Pice Move not found');
		d = d[kind];

		switch (kind) {
			case 'Piece':
				super({
					piece: 'str',
					from: 'str',
					to: 'str',
					capture: 'optstr',
					promotion: 'optstr'
				}, d);
				break;
			case 'EnPassant':
				super({
					from: 'str',
					to: 'str'
				}, d);
				break;
			case 'Castle':
				super({
					fromKing: 'str',
					toKing: 'str',
					fromRook: 'str',
					toRook: 'str'
				}, d);
				break;
		}

		this.kind = kind;
	}

	// on castling returns the king
	fromSquare() {
		switch (this.kind) {
			case 'Piece':
			case 'EnPassant':
				return this.from;
			case 'Castle':
				return this.fromKing;
		}
	}

	toSquare() {
		switch (this.kind) {
			case 'Piece':
			case 'EnPassant':
				return this.to;
			case 'Castle':
				return this.toKing;
		}
	}

	toJSON() {
		const obj = {};
		obj[this.kind] = {};
		Object.assign(obj[this.kind], this);
		return obj;
	}
}

const SQUARES = [
	'A8', 'B8', 'C8', 'D8', 'E8', 'F8', 'G8', 'H8',
	'A7', 'B7', 'C7', 'D7', 'E7', 'F7', 'G7', 'H7',
	'A6', 'B6', 'C6', 'D6', 'E6', 'F6', 'G6', 'H6',
	'A5', 'B5', 'C5', 'D5', 'E5', 'F5', 'G5', 'H5',
	'A4', 'B4', 'C4', 'D4', 'E4', 'F4', 'G4', 'H4',
	'A3', 'B3', 'C3', 'D3', 'E3', 'F3', 'G3', 'H3',
	'A2', 'B2', 'C2', 'D2', 'E2', 'F2', 'G2', 'H2',
	'A1', 'B1', 'C1', 'D1', 'E1', 'F1', 'G1', 'H1'
];

export function squareToIndex(square) {
	return SQUARES.indexOf(square);
}

export function indexToSquare(index) {
	return SQUARES[index];
}

export function indexToXY(index) {
	return [index % 8, Math.floor(index / 8)];
}

export function XYToIndex(x, y) {
	return y * 8 + x;
}