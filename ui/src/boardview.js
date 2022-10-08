import Sprite from './lib/sprite.js';
import { XYToIndex, indexToSquare, indexToXY, squareToIndex } from './api/data.js';
import { availableMoves } from './api/api.js';
import { range } from 'fire/util.js';
import Listeners from 'fire/util/listeners.js';

const pieceSprite = new Sprite('/img/piece_sprite.svg', 45, 45);

const pieceSpriteLookup = {
	'WhiteKing': [0, 0],
	'WhiteQueen': [1, 0],
	'WhiteBishop': [2, 0],
	'WhiteKnight': [3, 0],
	'WhiteRook': [4, 0],
	'WhitePawn': [5, 0],
	'BlackKing': [0, 1],
	'BlackQueen': [1, 1],
	'BlackBishop': [2, 1],
	'BlackKnight': [3, 1],
	'BlackRook': [4, 1],
	'BlackPawn': [5, 1]
}


export default class BoardView {
	constructor(ctx) {
		this.ctx = ctx;
		this.board = null;
		this.availableMoves = null;
		this.squareWidth = ctx.width / 8;

		// contains the index of the piece that is holded
		this.holdingPiece = null;
		this.holdingPieceRealXY = null;

		this.selectedPiece = null;
		this.moveToHint = range(0, 64).map(() => false);

		// piece drawing
		this.drawWidth = this.squareWidth * 0.85;
		this.drawPadding = (this.squareWidth - this.drawWidth) / 2;

		this.moveListeners = new Listeners;
	}

	// fn(x, y, i)
	loopSquare(fn) {
		for (let y = 0; y < 8; y++) {
			for (let x = 0; x < 8; x++) {
				fn(x, y, XYToIndex(x, y));
			}
		}
	}

	async updateBoard(board) {
		this.board = board;
		this.availableMoves = null;
		this.holdingPiece = null;
		this.holdingPieceRealXY = null;
		this.selectedPiece = null;
		this.moveToHint = range(0, 64).map(() => false);

		this.availableMoves = await availableMoves(board);
		console.log('moves', this.availableMoves);
	}

	onMove(fn) {
		return this.moveListeners.add(fn);
	}

	drawPieceReal(piece, side, x, y) {
		const [sX, sY] = pieceSpriteLookup[side + piece];
		pieceSprite.draw(
			sX, sY,
			this.ctx,
			x + this.drawPadding,
			y + this.drawPadding,
			this.drawWidth, this.drawWidth
		);
	}

	// piece Rook | , side: white
	drawPiece(piece, side, x, y) {
		this.drawPieceReal(
			piece,
			side,
			x * this.squareWidth,
			y * this.squareWidth
		);
	}

	draw() {
		this.ctx.fillStyle = '#eeeed2';
		this.ctx.fillRect(0, 0, this.ctx.width, this.ctx.height);

		this.ctx.fillStyle = '#769656';
		this.loopSquare((x, y, i) => {
			if ((x + (y % 2)) % 2 !== 0) {
				this.ctx.fillRect(
					x * this.squareWidth, y * this.squareWidth,
					this.squareWidth, this.squareWidth
				);
			}

			const piece = this.board.getPiece(i);
			if (piece && this.holdingPiece !== i)
				this.drawPiece(piece.kind, piece.side, x, y);
		});

		if (this.selectedPiece !== null) {
			this.ctx.fillStyle = 'rgba(0, 0, 0, .2)';
			this.moveToHint.forEach((showHint, i) => {
				if (showHint) {
					const [x, y] = indexToXY(i);
					this.ctx.fillCircle(
						x * this.squareWidth + this.squareWidth / 2,
						y * this.squareWidth + this.squareWidth / 2,
						this.squareWidth / 5
					)
				}
			});
		}

		if (this.holdingPiece !== null) {
			// draw the holding piece
			const piece = this.board.getPiece(this.holdingPiece);

			let [x, y] = this.holdingPieceRealXY;
			x -= this.squareWidth / 2;
			y -= this.squareWidth / 2;

			this.drawPieceReal(piece.kind, piece.side, x, y);
		}
	}

	// /// returns the square at the coordinates (which are the canvas xy)
	// squareAtRealXY(x, y) {
		
	// 	return indexToSquare(XYToIndex(x, y));
	// }

	mouseDown(rX, rY) {
		let x = Math.floor(rX / this.squareWidth);
		let y = Math.floor(rY / this.squareWidth);

		const index = XYToIndex(x, y);

		// if (this.selectedPiece == index) {
		// 	// just unselect
		// 	this.moveToHint = range(0, 64).map(() => false);
		// }

		this.selectedPiece = null;
		this.moveToHint = range(0, 64).map(() => false);

		let pieceFound = false;

		// calc hints
		switch (this.availableMoves.kind) {
			case 'Piece':
				this.availableMoves.moves.forEach(m => {
					const from = squareToIndex(m.fromSquare());
					if (from == index) {
						const toIndex = squareToIndex(m.toSquare());
						this.moveToHint[toIndex] = true;
						pieceFound = true;
					}
				});
				break;
			case 'Duck':
				throw new Error('todo duck');
				// this.availableMoves.squares.forEach(s => {
				// 	const idx = squareToIndex(s);
				// 	this.moveToHint[idx] = true;
				// });
				break;
		}

		if (!pieceFound)
			return;

		this.selectedPiece = index;

		this.holdingPiece = index;
		this.holdingPieceRealXY = [rX, rY];

		// this.holdingPiece = 
		// check if the move exists
		// and then show hints for that piece
	}

	mouseUp(rX, rY) {
		const prevHolding = this.holdingPiece;

		this.holdingPiece = null;
		this.holdingPieceRealXY = null;

		let x = Math.floor(rX / this.squareWidth);
		let y = Math.floor(rY / this.squareWidth);

		const index = XYToIndex(x, y);

		if (!this.moveToHint[index])
			return;

		switch (this.availableMoves.kind) {
			case 'Piece':
				const move = this.availableMoves.moves.find(m => {
					return squareToIndex(m.fromSquare()) == prevHolding &&
					squareToIndex(m.toSquare()) == index;
				});

				if (move)
					this.moveListeners.trigger(move);
				break;
			case 'Duck':
				throw new Error('todo duck');
				// this.availableMoves.squares.forEach(s => {
				// 	const idx = squareToIndex(s);
				// 	this.moveToHint[idx] = true;
				// });
				break;
		}
	}

	mouseMove(x, y) {
		if (this.holdingPiece !== null)
			this.holdingPieceRealXY = [x, y];
	}
}