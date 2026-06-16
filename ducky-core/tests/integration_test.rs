#![allow(clippy::panic)]

use ducky_core::DuckyCompiler;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

fn compile_and_compare(test_name: &str) {
    let input_path = PathBuf::from(format!("tests/{}.txt", test_name));
    let official_path = PathBuf::from(format!("tests/{}official.bin", test_name));

    let source = fs::read_to_string(&input_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", input_path.display(), e));

    let official_bytes = fs::read(&official_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", official_path.display(), e));

    let mut compiler = DuckyCompiler::new(None);
    let compiled_bytes = compiler
        .compile(&source)
        .unwrap_or_else(|e| panic!("Compilation failed for {}: {}", test_name, e));

    let rust_hash = compute_sha256(&compiled_bytes);
    let official_hash = compute_sha256(&official_bytes);

    assert_eq!(
        compiled_bytes.len(),
        official_bytes.len(),
        "{}: Size mismatch - Rust: {} bytes, Official: {} bytes",
        test_name,
        compiled_bytes.len(),
        official_bytes.len()
    );

    assert_eq!(
        rust_hash, official_hash,
        "{}: Hash mismatch\n  Rust:     {}\n  Official: {}",
        test_name, rust_hash, official_hash
    );

    println!("✓ {} passed (SHA256: {})", test_name, rust_hash);
}

#[test]
fn test_test1_parity() {
    compile_and_compare("test1");
}

#[test]
fn test_test2_parity() {
    compile_and_compare("test2");
}

#[test]
fn test_test3_parity() {
    compile_and_compare("test3");
}

#[test]
fn test_test4_parity() {
    compile_and_compare("test4");
}

#[test]
fn test_test5_parity() {
    compile_and_compare("test5");
}

#[test]
fn test_test6_parity() {
    compile_and_compare("test6");
}

#[test]
fn test_test7_parity() {
    compile_and_compare("test7");
}

#[test]
fn test_all_parity() {
    let tests = [
        "test1", "test2", "test3", "test4", "test5", "test6", "test7",
    ];

    for test_name in &tests {
        compile_and_compare(test_name);
    }

    println!("\n✓ All {} tests passed!", tests.len());
}
