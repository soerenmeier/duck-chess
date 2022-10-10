<script>
	import BoardView from './board.svelte';
	import { Board } from './api/data.js';
	import { newBoard } from './api/api.js';

	let board = Board.empty();
	let boardInited = false;

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
		console.log('eval', board);
	}

	$: evaluateBoard(board);

	load();
</script>

<main id="duck-chess">
	<BoardView bind:board={board} />
</main>

<style>
</style>