use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regex_benchmark::{read_test_string_from_file, ASCII_REGEX, UNICODE_REGEX};
use std::time::Instant;

/// Unicode対応の正規表現の処理時間を測定する関数
fn benchmark_unicode_regex(c: &mut Criterion) {
    let file_path = "test_input.txt";
    match read_test_string_from_file(file_path) {
        Ok(input) => {
            c.bench_function("unicode_regex", |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = std::time::Duration::ZERO;
                    for _ in 0..iters {
                        let start = Instant::now();
                        UNICODE_REGEX.is_match(black_box(&input)); // 処理対象
                        total_duration += start.elapsed();
                    }
                    total_duration
                });
            });
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }
}

/// ASCII専用の正規表現の処理時間を測定する関数
fn benchmark_ascii_regex(c: &mut Criterion) {
    let file_path = "test_input.txt";
    match read_test_string_from_file(file_path) {
        // テストデータを生成
        Ok(input) => {
            c.bench_function("ascii_regex", |b| {
                b.iter_custom(|iters| {
                    let mut total_duration = std::time::Duration::ZERO;
                    for _ in 0..iters {
                        let start = Instant::now();
                        ASCII_REGEX.is_match(black_box(&input)); // 処理対象
                        total_duration += start.elapsed();
                    }
                    total_duration
                });
            });
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }
    
}

// ベンチマークグループを定義
criterion_group!(benches, benchmark_unicode_regex, benchmark_ascii_regex);
criterion_main!(benches);
