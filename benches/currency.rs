use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::thread_rng;
use rand::Rng;
use std::time::Duration;

fn generate_random_f64_vec(size: u32) -> Vec<f64> {
    (1..=size)
        .map(|_| thread_rng().gen_range(1.0..20.0))
        .collect()
}

fn generate_random_string_vec(size: u32) -> Vec<String> {
    (1..=size)
        .map(|_| thread_rng().gen_range(1.0..20.0).to_string())
        .collect()
}

//----------------------------------------------------------//

fn add_string(values: Vec<String>) {
    let mut currency = currency_rs::Currency::new_string("0.0", None).unwrap();

    for value in values {
        currency = currency.add_string(value.as_str()).unwrap();
    }
}

fn add_string_benchmark(c: &mut Criterion) {
    c.bench_function("add string benchmark", |b| {
        b.iter(|| add_string(black_box(generate_random_string_vec(1_000))))
    });
}

//----------------------------------------------------------//

fn add_string_and_format(values: Vec<String>) -> String {
    let mut currency = currency_rs::Currency::new_string("0.0", None).unwrap();
    let mut total: String = String::from("0.0");

    for value in values {
        currency = currency.add_string(value.as_str()).unwrap();
        total = currency.format();
    }

    total
}

fn add_string_and_format_benchmark(c: &mut Criterion) {
    c.bench_function("add string and format benchmark", |b| {
        b.iter(|| add_string_and_format(black_box(generate_random_string_vec(1_000))))
    });
}

//----------------------------------------------------------//

fn add_float(values: Vec<f64>) {
    let mut total = currency_rs::Currency::new_float(0.0, None);
    for value in values {
        total = total.add(value);
    }
}

fn add_float_benchmark(c: &mut Criterion) {
    c.bench_function("add float benchmark", |b| {
        b.iter(|| add_float(black_box(generate_random_f64_vec(1_000))))
    });
}

criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(100));
  targets = add_string_benchmark, add_string_and_format_benchmark, add_float_benchmark
}

criterion_main!(benches);
