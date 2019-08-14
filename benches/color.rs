#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

use resume::models::color::Color;
use std::str::FromStr;

fn color_from_string_benchmark(c: &mut Criterion) {
    c.bench_function("Color::from_str", |b| b.iter(||
        Color::from_str(black_box("123, 42, 23"))
    ));
}

criterion_group!(color, color_from_string_benchmark);
criterion_main!(color);
