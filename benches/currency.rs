use criterion::{black_box, criterion_group, criterion_main, Criterion};
use currency_rs::{Currency, CurrencyOpts};
use rand::thread_rng;
use rand::Rng;
use std::time::Duration;

fn random_floats(n: u32) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..n).map(|_| rng.gen_range(1.0..1000.0)).collect()
}

fn random_strings(n: u32) -> Vec<String> {
    let mut rng = thread_rng();
    (0..n)
        .map(|_| format!("${:.2}", rng.gen_range(1.0..1000.0)))
        .collect()
}

fn bench_parse(c: &mut Criterion) {
    let inputs: Vec<String> = random_strings(500);
    c.bench_function("parse", |b| {
        b.iter(|| {
            for s in &inputs {
                black_box(black_box(s).parse::<Currency>()).unwrap();
            }
        })
    });
}

fn bench_from(c: &mut Criterion) {
    let inputs = random_floats(500);
    c.bench_function("from", |b| {
        b.iter(|| {
            for v in &inputs {
                black_box(Currency::from(black_box(*v)));
            }
        })
    });
}

fn bench_format(c: &mut Criterion) {
    let inputs: Vec<Currency> = random_floats(500)
        .iter()
        .map(|&v| Currency::from(v))
        .collect();
    c.bench_function("format", |b| {
        b.iter(|| {
            for cur in &inputs {
                black_box(cur.format());
            }
        })
    });
}

fn bench_format_vedic(c: &mut Criterion) {
    let opts = CurrencyOpts::new().use_vedic(true);
    let inputs: Vec<Currency> = random_floats(500).iter().map(|&v| opts.from(v)).collect();
    c.bench_function("format_vedic", |b| {
        b.iter(|| {
            for cur in &inputs {
                black_box(cur.format());
            }
        })
    });
}

fn bench_operator_add(c: &mut Criterion) {
    let vals = random_floats(500);
    c.bench_function("operator_add", |b| {
        b.iter(|| {
            let mut cur = Currency::from(0.0);
            for &v in &vals {
                cur = black_box(cur + v);
            }
            black_box(cur);
        })
    });
}

fn bench_operator_sub(c: &mut Criterion) {
    let vals = random_floats(500);
    c.bench_function("operator_sub", |b| {
        b.iter(|| {
            let mut cur = Currency::from(1_000_000.0);
            for &v in &vals {
                cur = black_box(cur - v);
            }
            black_box(cur);
        })
    });
}

fn bench_operator_mul(c: &mut Criterion) {
    let vals = random_floats(500);
    c.bench_function("operator_mul", |b| {
        b.iter(|| {
            let mut cur = Currency::from(1.0);
            for &v in &vals {
                cur = black_box(cur * v);
            }
            black_box(cur);
        })
    });
}

fn bench_operator_div(c: &mut Criterion) {
    let vals: Vec<f64> = random_floats(500).iter().map(|&v| v.max(0.01)).collect();
    c.bench_function("operator_div", |b| {
        b.iter(|| {
            let mut cur = Currency::from(1_000_000.0);
            for &v in &vals {
                cur = black_box(cur / v);
            }
            black_box(cur);
        })
    });
}

fn bench_distribute(c: &mut Criterion) {
    c.bench_function("distribute", |b| {
        b.iter(|| {
            let cur = Currency::from(black_box(100.0));
            black_box(cur.distribute(black_box(6)));
        })
    });
}

fn bench_e2e(c: &mut Criterion) {
    let inputs: Vec<String> = random_strings(200);
    c.bench_function("e2e_parse_add_format", |b| {
        b.iter(|| {
            let mut cur = "$0.00".parse::<Currency>().unwrap();
            for s in &inputs {
                let item = black_box(s.as_str()).parse::<Currency>().unwrap();
                cur = black_box(cur + item);
            }
            black_box(cur.format());
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);
    targets =
        bench_parse,
        bench_from,
        bench_format,
        bench_format_vedic,
        bench_operator_add,
        bench_operator_sub,
        bench_operator_mul,
        bench_operator_div,
        bench_distribute,
        bench_e2e,
}

criterion_main!(benches);
