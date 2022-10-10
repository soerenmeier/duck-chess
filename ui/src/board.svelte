<script>
	import { timeout } from 'fire/util.js';
	import Context2d from 'fire/dom/context2d.js';
	import BoardView from './boardview.js';
	import { applyMove } from './api/api.js';

	// should be a Board (see api)
	export let board;

	let view;
	let canvas;
	

	async function newCanvas(el) {
		canvas = el;
		const ctx = new Context2d(el);
		ctx.updateSize(600, 600);

		view = new BoardView(ctx, board);

		view.onMove(async ([kind, move]) => {
			board = await applyMove(kind, move, view.board);
		})

		// load sprite
		await timeout(300);

		await view.updateBoard(board);

		requestAnimationFrame(draw);
	}

	$: view ? view.updateBoard(board) : [];

	function draw() {
		view.draw();

		requestAnimationFrame(draw);
	}

	// click handling
	let mouseDown = false;

	// might return [null, null] if the xy is invalid
	function getMouseCanvasXY(ev) {
		const offset = canvas.getBoundingClientRect();
		let x = ev.clientX - offset.left;
		let y = ev.clientY - offset.top;

		if (x > offset.width || y > offset.height) {
			x = null;
			y = null;
		}

		return [x, y];
	}

	function onMouseDown(e) {
		mouseDown = true;
		const [x, y] = getMouseCanvasXY(e);

		if (x !== null)
			view.mouseDown(x, y);
	}

	function onMouseUp(e) {
		mouseDown = false;
		const [x, y] = getMouseCanvasXY(e);

		if (x !== null)
			view.mouseUp(x, y);
	}

	function onMouseMove(e) {
		if (!mouseDown)
			return;

		const [x, y] = getMouseCanvasXY(e);

		if (x !== null)
			view.mouseMove(x, y);
	}

</script>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} />

<canvas
	id="canvas"
	use:newCanvas
	on:mousedown={onMouseDown}
/>