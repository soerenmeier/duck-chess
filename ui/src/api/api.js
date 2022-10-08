
import Data from 'fire/data/data.js';
import Api from 'fire/api/api.js';
import { Iteration } from './data.js';

const api = new Api('http://127.0.0.1:1555/api');

export class CurrentState extends Data {
	constructor(d) {
		super({
			ai: 'str',
			stats: [RunStat]
		}, d);
	}
}

export async function currentState() {
	const d = await api.request('GET', '/current-state');
	return new CurrentState(d);
}