<script>
	import BoardView from './board.svelte';
	import { Board } from './api/data.js';
	import { newBoard } from './api/api.js';

	let board = Board.empty();

	async function load() {
		board = await newBoard();
		// const moves = await availableMoves(board);
		// console.log('moves', moves);
	}

	load();
</script>

<main id="duck-chess">
	<BoardView {board} />
</main>

<style>
</style>