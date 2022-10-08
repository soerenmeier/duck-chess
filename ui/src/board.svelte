<script>
	import { timeout } from 'fire/util.js';
	import Context2d from 'fire/dom/context2d.js';
	import BoardView from './boardview.js';

	// should be a Board (see api)
	export let board;

	let view;
	let canvas;
	

	async function newCanvas(el) {
		canvas = el;
		const ctx = new Context2d(el);
		ctx.updateSize(600, 600);

		view = new BoardView(ctx, board);

		view.onMove(move => {
			console.log('wan\'t to move', move);
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

	function getMouseCanvasXY(ev) {
		const offset = canvas.getBoundingClientRect();
		const x = ev.clientX - offset.left;
		const y = ev.clientY - offset.top;

		return [x, y];
	}

	function onMouseDown(e) {
		mouseDown = true;
		const [x, y] = getMouseCanvasXY(e);

		view.mouseDown(x, y);

		// console.log(view.squareAtRealXY(x, y));
	}
	function onMouseUp(e) {
		mouseDown = false;
		const [x, y] = getMouseCanvasXY(e);

		view.mouseUp(x, y);
	}

	function onMouseMove(e) {
		if (!mouseDown)
			return;

		const [x, y] = getMouseCanvasXY(e);

		view.mouseMove(x, y);
	}

</script>

<svelte:window on:mousemove={onMouseMove} on:mouseup={onMouseUp} />

<canvas
	id="canvas"
	use:newCanvas
	on:mousedown={onMouseDown}
/>