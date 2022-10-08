
import Data from 'fire/data/data.js';

export class AnimalProps extends Data {
	constructor(d) {
		super({
			hunger: 'float',
			speed: 'float',
			maxSpeed: 'float',
			health: 'float',
			stamina: 'float',
			posX: 'int',
			posY: 'int',
			rotation: 'float',
			rotationSpeed: 'float',
			ear1: 'float',
			ear2: 'float',
			radius: 'int',
			hearingDistance: 'int',
			viewingDistance: 'int',
			viewingAngle: 'float'
		}, d);
	}
}