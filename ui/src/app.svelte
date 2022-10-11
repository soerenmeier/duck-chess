<script>
	import BoardView from './board.svelte';
	import { Board } from './api/data.js';
	import { newBoard, evaluateBoard as evaluateBoardApi } from './api/api.js';

	let board = Board.empty();
	let boardInited = false;

	// eval
	let depth = 2;
	let loadingEval = false;
	let evalMoves = [];

	async function load() {
		console.log('load');
		board = await newBoard();
		boardInited = true;
		// const moves = await availableMoves(board);
		// console.log('moves', moves);
	}

	async function evaluateBoard(board) {
		if (board.movedPiece || !boardInited)
			return;
		loadingEval = true;
		evalMoves = await evaluateBoardApi(board, depth);
		loadingEval = false;
		console.log('moves', moves);
	}

	function displayMove(move) {
		let pieceText;
		switch (move.piece.kind) {
			case 'Piece':
				pieceText = `${move.piece.piece} ${move.piece.from}>${move.piece.to}`;
				break;
			case 'EnPassant':
				pieceText = `Pawn ${move.piece.from}>${move.piece.to}`;
				break;
			case 'Castle':
				pieceText = `Castle ${move.piece.fromKing}>${move.piece.toKing}`;
				break;
		}
		return `${move.side} ${pieceText}, Duck ${move.duck}`;
	}

	load();
</script>

<main id="duck-chess">
	<BoardView bind:board={board} />
	<div class="evaluation">
		<h1>Eval</h1>
		<label for="depth">Depth</label>
		<input type="number" name="depth" id="depth" bind:value={depth}>
		<button on:click={() => evaluateBoard(board)}>Eval</button>
		<h2>Moves</h2>
		{#if loadingEval}
			<p>Loading..</p>
		{/if}
		<div class="list">
			{#each evalMoves as [score, move]}
				<p>{score}: {displayMove(move)}</p>
			{/each}
		</div>
	</div>
</main>

<style>
	#duck-chess {
		display: grid;
		grid-template-columns: auto 1fr;
	}

	.evaluation {
		padding: 10px;
	}
</style>