extern crate criterion;
extern crate num_bigint;
extern crate rug;
// extern crate ibig;
// extern crate ramp;

use criterion::{criterion_group, criterion_main, Criterion, black_box};
use num_bigint::BigUint as NumBigUint;
use rug::{Complete, Integer as RugInteger};
use ibig::IBig;
use ramp::Int;

const LARGE_NUM_STR1: &str = "123213213123213213123213213123213213123213213123213213123213213123213213123213213123213213123213213123213213123213123213213123213213123213123213213123213";
const LARGE_NUM_STR2: &str = "987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210987654";
const LARGE_NUM_STR3: &str = "321098765432109876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210987654321098765432109876543210987654";

fn bench_num_bigint(c: &mut Criterion) {
    let a = NumBigUint::parse_bytes(LARGE_NUM_STR1.as_bytes(), 10).unwrap();
    let b = NumBigUint::parse_bytes(LARGE_NUM_STR2.as_bytes(), 10).unwrap();
    let q = NumBigUint::parse_bytes(LARGE_NUM_STR3.as_bytes(), 10).unwrap();
    let mut r = a.clone();

    c.bench_function("num-bigint mul", |bencher| {
        bencher.iter(|| {
            let t = black_box(&a) * black_box(&b);
            let u = black_box(&t) % black_box(&q);
            r = u;
        });
    });
}

fn bench_rug(c: &mut Criterion) {
    let a = RugInteger::from_str_radix(LARGE_NUM_STR1, 10).unwrap();
    let b = RugInteger::from_str_radix(LARGE_NUM_STR2, 10).unwrap();
    let q = RugInteger::from_str_radix(LARGE_NUM_STR3, 10).unwrap();
    let mut r = a.clone();

    c.bench_function("rug mul", |bencher| {
        bencher.iter(|| {
            let t = (black_box(&r) * black_box(&b)).complete();
            let u = black_box(&t) % black_box(&q);
            r = u.complete();
        });
    });
}

// fn bench_ibig(c: &mut Criterion) {
//     let a = IBig::from_str_radix(LARGE_NUM_STR1, 10).unwrap();
//     let b = IBig::from_str_radix(LARGE_NUM_STR2, 10).unwrap();
//     let q = IBig::from_str_radix(LARGE_NUM_STR3, 10).unwrap();
//     let mut r = a.clone();

//     c.bench_function("ibig mul", |bencher| {
//         bencher.iter(|| {
//             let t = black_box(&a) * black_box(&b);
//             let u = black_box(&t) % black_box(&q);
//             r = u;
//         });
//     });
// }

// fn bench_ramp(c: &mut Criterion) {
//     let a = Int::from_str_radix(LARGE_NUM_STR1, 10).unwrap();
//     let b = Int::from_str_radix(LARGE_NUM_STR2, 10).unwrap();
//     let q = Int::from_str_radix(LARGE_NUM_STR3, 10).unwrap();
//     let mut r = a.clone();

//     c.bench_function("ramp mul", |bencher| {
//         bencher.iter(|| {
//             let t = black_box(&a) * black_box(&b);
//             let u = black_box(&t) % black_box(&q);
//             r = u;
//         });
//     });
// }

criterion_group!(benches, bench_num_bigint, bench_rug);
criterion_main!(benches);
