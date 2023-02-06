use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use crate::cache::*;


// fn normal(tt: TranspositionTable) -> {
//     let entry = &mut tt.table[hash as usize % tt.size];
//     entry.hash = hash;
//     entry.score = score;
//     entry.best_move = best_move;
//     entry.depth = depth;
//     entry.flag = flag;
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);