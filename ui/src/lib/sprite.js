
export default class Sprite {
	// w, h of the square
	constructor(img, w, h) {
		this.img = new Image;
		this.img.src = img;
		this.w = w;
		this.h = h;
	}

	draw(x, y, ctx, dX, dY, dW, dH) {
		ctx.drawImage(
			this.img,
			x * this.w, y * this.h,
			this.w, this.h,
			dX, dY,
			dW, dH
		);
	}
}