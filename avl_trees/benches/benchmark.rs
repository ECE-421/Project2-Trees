use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avl_trees::tree::Tree;

fn criterion_benchmark(c: &mut Criterion) {
    for &size in &[10000, 40000, 70000, 100000, 130000] {
        let mut tree: Tree<i64> = Tree::new();
        let values: Vec<i64> = (0..size).collect();
        c.bench_function(&format!("insert_{}", size), |b| {
            b.iter(|| {
                for &value in &values {
                    tree.insert(black_box(value));
                }
            });
        });

        for &value in &values {
            tree.insert(black_box(value));
        }

        c.bench_function(&format!("search_{}", size), |b| {
            b.iter(|| {
                for &value in &values[..(size/10).try_into().unwrap()] {
                    tree.search(black_box(value));
                }
            });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
