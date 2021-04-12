use criterion::{black_box, criterion_group, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);

fn main() {
    #[cfg(target_os = "ios")]
    {
        use tai_util::ios::user_documents;
        let path = user_documents().join("target");
        std::fs::create_dir(&path);
        std::env::set_var("CRITERION_HOME", path);
    }

    benches();

    criterion::Criterion::default()
        .configure_from_args()
        .final_summary();
}
