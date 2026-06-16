#![allow(clippy::panic, clippy::unwrap_used)]

use criterion::{Criterion, criterion_group, criterion_main};
use ducky_core::DuckyCompiler;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;

fn load_tests() -> Vec<(String, String)> {
    let names = [
        "test1", "test2", "test3", "test4", "test5", "test6", "test7",
    ];
    names
        .iter()
        .map(|name| {
            let path = PathBuf::from(format!("tests/{}.txt", name));
            let source = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
            (name.to_string(), source)
        })
        .collect()
}

fn bench_compile(c: &mut Criterion) {
    let tests = load_tests();

    let mut group = c.benchmark_group("compile");
    for (name, source) in &tests {
        group.bench_function(name, |b| {
            b.iter(|| {
                let mut compiler = DuckyCompiler::new(None);
                let out = compiler.compile(black_box(source)).unwrap();
                black_box(out);
            });
        });
    }
    group.finish();

    c.bench_function("compile_all", |b| {
        b.iter(|| {
            for (_name, source) in &tests {
                let mut compiler = DuckyCompiler::new(None);
                let out = compiler.compile(black_box(source)).unwrap();
                black_box(out);
            }
        });
    });
}

criterion_group!(benches, bench_compile);
criterion_main!(benches);
