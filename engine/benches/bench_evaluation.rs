use engine::logic::ComputedBoard;

use criterion::{criterion_group, criterion_main, Criterion};


fn evaluation_benchmark(c: &mut Criterion) {
	let mut board = ComputedBoard::new();

	// c.bench_function("eval depth 1", |b| b.iter(|| board.evaluate(1)));
	c.bench_function("eval depth 2", |b| b.iter(|| board.evaluate(2)));
}

criterion_group!(benches, evaluation_benchmark);
criterion_main!(benches);