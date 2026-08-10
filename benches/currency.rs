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

// --------------------------------------------------
// Parse — exercises regex vs no-regex string parsing
// --------------------------------------------------

fn bench_new_string(c: &mut Criterion) {
    let inputs: Vec<String> = random_strings(500);
    c.bench_function("new_string", |b| {
        b.iter(|| {
            for s in &inputs {
                black_box(Currency::new_string(black_box(s), None)).unwrap();
            }
        })
    });
}

fn bench_new_float(c: &mut Criterion) {
    let inputs = random_floats(500);
    c.bench_function("new_float", |b| {
        b.iter(|| {
            for v in &inputs {
                black_box(Currency::new_float(black_box(*v), None));
            }
        })
    });
}

// --------------------------------------------------
// Format — exercises regex vs no-regex dollar grouping
// --------------------------------------------------

fn bench_format(c: &mut Criterion) {
    let inputs: Vec<Currency> = random_floats(500)
        .iter()
        .map(|&v| Currency::new_float(v, None))
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
    let opts = CurrencyOpts::new().set_use_vedic(true);
    let inputs: Vec<Currency> = random_floats(500)
        .iter()
        .map(|&v| Currency::new_float(v, Some(opts.clone())))
        .collect();
    c.bench_function("format_vedic", |b| {
        b.iter(|| {
            for cur in &inputs {
                black_box(cur.format());
            }
        })
    });
}

// --------------------------------------------------
// Arithmetic
// --------------------------------------------------

fn bench_add(c: &mut Criterion) {
    let vals = random_floats(500);
    c.bench_function("add", |b| {
        b.iter(|| {
            let mut cur = Currency::new_float(0.0, None);
            for &v in &vals {
                cur = black_box(cur.add(black_box(v)));
            }
            black_box(cur);
        })
    });
}

fn bench_add_string(c: &mut Criterion) {
    let vals = random_strings(500);
    c.bench_function("add_string", |b| {
        b.iter(|| {
            let mut cur = Currency::new_string("0.0", None).unwrap();
            for s in &vals {
                cur = black_box(cur.add_string(black_box(s.as_str())).unwrap());
            }
            black_box(cur);
        })
    });
}

fn bench_subtract(c: &mut Criterion) {
    let vals = random_floats(500);
    c.bench_function("subtract", |b| {
        b.iter(|| {
            let mut cur = Currency::new_float(1_000_000.0, None);
            for &v in &vals {
                cur = black_box(cur.subtract(black_box(v)));
            }
            black_box(cur);
        })
    });
}

fn bench_multiply(c: &mut Criterion) {
    let vals = random_floats(500);
    c.bench_function("multiply", |b| {
        b.iter(|| {
            let mut cur = Currency::new_float(1.0, None);
            for &v in &vals {
                cur = black_box(cur.multiply(black_box(v)));
            }
            black_box(cur);
        })
    });
}

fn bench_divide(c: &mut Criterion) {
    let vals: Vec<f64> = random_floats(500).iter().map(|&v| v.max(0.01)).collect();
    c.bench_function("divide", |b| {
        b.iter(|| {
            let mut cur = Currency::new_float(1_000_000.0, None);
            for &v in &vals {
                cur = black_box(cur.divide(black_box(v)));
            }
            black_box(cur);
        })
    });
}

// --------------------------------------------------
// Operator overloads
// --------------------------------------------------

fn bench_operator_add(c: &mut Criterion) {
    let vals = random_floats(500);
    c.bench_function("operator_add", |b| {
        b.iter(|| {
            let mut cur = Currency::new_float(0.0, None);
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
            let mut cur = Currency::new_float(1_000_000.0, None);
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
            let mut cur = Currency::new_float(1.0, None);
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
            let mut cur = Currency::new_float(1_000_000.0, None);
            for &v in &vals {
                cur = black_box(cur / v);
            }
            black_box(cur);
        })
    });
}

// --------------------------------------------------
// Distribute
// --------------------------------------------------

fn bench_distribute(c: &mut Criterion) {
    c.bench_function("distribute", |b| {
        b.iter(|| {
            let cur = Currency::new_float(black_box(100.0), None);
            black_box(cur.distribute(black_box(6)));
        })
    });
}

// --------------------------------------------------
// End-to-end: parse → arithmetic → format
// --------------------------------------------------

fn bench_e2e(c: &mut Criterion) {
    let inputs: Vec<String> = random_strings(200);
    c.bench_function("e2e_parse_add_format", |b| {
        b.iter(|| {
            let mut cur = Currency::new_string("$0.00", None).unwrap();
            for s in &inputs {
                let item = Currency::new_string(black_box(s.as_str()), None).unwrap();
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
        bench_new_string,
        bench_new_float,
        bench_format,
        bench_format_vedic,
        bench_add,
        bench_add_string,
        bench_subtract,
        bench_multiply,
        bench_divide,
        bench_operator_add,
        bench_operator_sub,
        bench_operator_mul,
        bench_operator_div,
        bench_distribute,
        bench_e2e,
}

criterion_main!(benches);
