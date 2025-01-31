use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::{BTreeSet, HashSet};

fn add_to_vec(vec: &mut Vec<i32>, new_number: i32) {
    if !vec.contains(&new_number) {
        vec.push(new_number);
    }
}

fn add_to_hashset(set: &mut HashSet<i32>, new_number: i32) {
    set.insert(new_number);
}

fn add_to_btreeset(set: &mut BTreeSet<i32>, new_number: i32) {
    set.insert(new_number);
}

const DATA: &[i32] = &[4, 3, 1, 2];

fn benchmark_vec(c: &mut Criterion) {
    c.bench_function("Vec", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for &i in DATA {
                add_to_vec(&mut vec, black_box(i));
            }
        })
    });
}

fn benchmark_hashset(c: &mut Criterion) {
    c.bench_function("HashSet", |b| {
        b.iter(|| {
            let mut set = HashSet::new();
            for &i in DATA {
                add_to_hashset(&mut set, black_box(i));
            }
        })
    });
}

fn benchmark_btreeset(c: &mut Criterion) {
    c.bench_function("BTreeSet", |b| {
        b.iter(|| {
            let mut set = BTreeSet::new();
            for &i in DATA {
                add_to_btreeset(&mut set, black_box(i));
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_vec,
    benchmark_hashset,
    benchmark_btreeset
);
criterion_main!(benches);
