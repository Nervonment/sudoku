use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::{
    generator::random_sudoku_puzzle_ultimate,
    state::full_state::FullState,
    techniques::{
        fish::{Jellyfish, Swordfish, XWing},
        hidden_subsets::HiddenPair,
        locked_candidates::{Claiming, Pointing},
        naked_subsets::{NakedPair, NakedSubset},
        Technique,
    },
};

fn benchmarks(c: &mut Criterion) {
    let puzzle = random_sudoku_puzzle_ultimate();
    let state = FullState::from(puzzle);

    c.bench_function("Hidden Pair", |b| {
        b.iter(|| {
            HiddenPair::default().analyze(&state);
        })
    });
    c.bench_function("Naked Pair", |b| {
        b.iter(|| {
            NakedPair::default().analyze(&state);
        })
    });
    c.bench_function("Pointing", |b| {
        b.iter(|| {
            Pointing::default().analyze(&state);
        })
    });
    c.bench_function("Claiming", |b| {
        b.iter(|| {
            Claiming::default().analyze(&state);
        })
    });
    c.bench_function("Naked Subset", |b| {
        b.iter(|| {
            NakedSubset::default().analyze(&state);
        })
    });
    c.bench_function("X-Wing", |b| {
        b.iter(|| {
            XWing::default().analyze(&state);
        })
    });
    c.bench_function("Swordfish", |b| {
        b.iter(|| {
            Swordfish::default().analyze(&state);
        })
    });
    c.bench_function("Jellyfish", |b| {
        b.iter(|| {
            Jellyfish::default().analyze(&state);
        })
    });
}

criterion_group! {
    name=benches;
    config=Criterion::default().significance_level(0.1).sample_size(1000);
    targets=benchmarks
}
criterion_main!(benches);
