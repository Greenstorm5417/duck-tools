#![allow(clippy::unwrap_used)]

use ducky_core::DuckyCompiler;
use std::hint::black_box;

const SOURCES: [&str; 7] = [
    include_str!("../tests/test1.txt"),
    include_str!("../tests/test2.txt"),
    include_str!("../tests/test3.txt"),
    include_str!("../tests/test4.txt"),
    include_str!("../tests/test5.txt"),
    include_str!("../tests/test6.txt"),
    include_str!("../tests/test7.txt"),
];

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000);

    let only: Option<usize> = std::env::args().nth(2).and_then(|s| s.parse().ok());

    let mut total = 0usize;
    for _ in 0..iterations {
        for (idx, source) in SOURCES.iter().enumerate() {
            if let Some(o) = only
                && o != idx
            {
                continue;
            }
            let mut compiler = DuckyCompiler::new(None);
            let out = compiler.compile(black_box(source)).unwrap();
            total = total.wrapping_add(out.len());
        }
    }
    println!("done, checksum={}", total);
}
