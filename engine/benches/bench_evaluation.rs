use engine::logic::ComputedBoard;

use criterion::{criterion_group, criterion_main, Criterion};


fn evaluation_benchmark(c: &mut Criterion) {
	let mut board = ComputedBoard::new();

	// c.bench_function("eval depth 1", |b| b.iter(|| board.evaluate(1)));
	c.bench_function("eval depth 1", |b| b.iter(|| board.evaluate(1)));
	c.bench_function("eval depth 2", |b| b.iter(|| board.evaluate(2)));
	c.bench_function("eval depth 3", |b| b.iter(|| board.evaluate(3)));
	c.bench_function("eval depth 4", |b| b.iter(|| board.evaluate(4)));
}

criterion_group! {
	name = benches;
	config = Criterion::default().sample_size(10);
	targets = evaluation_benchmark
}
criterion_main!(benches);