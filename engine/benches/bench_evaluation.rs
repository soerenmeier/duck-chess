use engine::logic::ComputedBoard;

use criterion::{criterion_group, criterion_main, Criterion};


fn fast_benchmark(c: &mut Criterion) {
	let board = ComputedBoard::new();

	c.bench_function("eval depth 2", |b| b.iter(|| board.evaluate(2)));
	c.bench_function("eval depth 3", |b| b.iter(|| board.evaluate(3)));
}

fn slow_benchmark(c: &mut Criterion) {
	let board = ComputedBoard::new();
	c.bench_function("eval depth 4", |b| b.iter(|| board.evaluate(4)));
}

criterion_group! {
	name = fast_benches;
	config = Criterion::default().sample_size(200);
	targets = fast_benchmark
}
criterion_group! {
	name = slow_benches;
	config = Criterion::default().sample_size(10);
	targets = slow_benchmark
}
criterion_main!(fast_benches, slow_benches);