use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jamliquor::state::State;
use jamliquor::Importer;

fn benchmark_block_import(c: &mut Criterion) {
    let mut group = c.benchmark_group("Block Import");
    let mut importer = Importer::new();
    let test_vector_path = std::path::PathBuf::from("tests/vectors/codec/data/block.json");

    group.bench_function("import single block", |b| {
        b.iter(|| {
            let _block = black_box(importer.import_block(&test_vector_path)).unwrap();
        })
    });

    group.finish();
}

fn benchmark_state_transition(c: &mut Criterion) {
    let mut group = c.benchmark_group("State Transition");
    let mut state = State::new();
    let mut importer = Importer::new();
    let test_vector_path = std::path::PathBuf::from("tests/vectors/codec/data/block.json");

    group.bench_function("apply block to state", |b| {
        b.iter(|| {
            let _block = black_box(importer.import_block(&test_vector_path)).unwrap();
            black_box(state.apply_block(&_block)).unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_block_import, benchmark_state_transition);
criterion_main!(benches);
