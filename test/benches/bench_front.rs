use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pareto::{Dominate, ParetoFront};
use rand::random;

#[derive(Dominate, Clone, Copy)]
struct A(usize);

impl A {
    fn new() -> Self {
        Self(random())
    }
}

#[derive(Dominate, Clone, Copy)]
struct B {
    a: usize,
    b: usize,
    c: usize,
}

impl B {
    fn new() -> Self {
        Self {
            a: random(),
            b: random(),
            c: random(),
        }
    }
}

#[derive(Dominate, Clone, Copy)]
struct C(usize, usize, usize, usize, usize, usize, usize); // 7 fields

impl C {
    fn new() -> Self {
        Self(
            random(),
            random(),
            random(),
            random(),
            random(),
            random(),
            random(),
        )
    }
}

pub fn bench_small(c: &mut Criterion) {
    c.bench_function("1d 100", |b| {
        let v = std::iter::from_fn(|| Some(A::new()))
            .take(100)
            .collect::<Vec<_>>();
        b.iter(|| {
            let mut front = ParetoFront::new();
            for &a in &v {
                front.push(black_box(a));
            }
        })
    });

    c.bench_function("3d 100", |b| {
        let v = std::iter::from_fn(|| Some(B::new()))
            .take(100)
            .collect::<Vec<_>>();
        b.iter(|| {
            let mut front = ParetoFront::new();
            for &a in &v {
                front.push(black_box(a));
            }
        })
    });

    c.bench_function("7d 100", |b| {
        let v = std::iter::from_fn(|| Some(C::new()))
            .take(100)
            .collect::<Vec<_>>();
        b.iter(|| {
            let mut front = ParetoFront::new();
            for &a in &v {
                front.push(black_box(a));
            }
        })
    });
}

pub fn bench_med(c: &mut Criterion) {
    c.bench_function("1d 1000", |b| {
        let v = std::iter::from_fn(|| Some(A::new()))
            .take(1000)
            .collect::<Vec<_>>();
        b.iter(|| {
            let mut front = ParetoFront::new();
            for &a in &v {
                front.push(black_box(a));
            }
        })
    });

    c.bench_function("3d 1000", |b| {
        let v = std::iter::from_fn(|| Some(B::new()))
            .take(1000)
            .collect::<Vec<_>>();
        b.iter(|| {
            let mut front = ParetoFront::new();
            for &a in &v {
                front.push(black_box(a));
            }
        })
    });

    c.bench_function("7d 1000", |b| {
        let v = std::iter::from_fn(|| Some(C::new()))
            .take(1000)
            .collect::<Vec<_>>();
        b.iter(|| {
            let mut front = ParetoFront::new();
            for &a in &v {
                front.push(black_box(a));
            }
        })
    });
}

criterion_group!(benches, bench_small, bench_med);
criterion_main!(benches);
