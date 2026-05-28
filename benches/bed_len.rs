use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rsomics_bed_len::lengths;
use std::io::Cursor;

fn make_fixture(n: usize) -> String {
    let mut s = String::with_capacity(n * 40);
    for i in 0..n {
        let start = i as u64 * 1000;
        let end = start + 500;
        s.push_str(&format!("chr1\t{start}\t{end}\tfeat{i}\n"));
    }
    s
}

fn bench_lengths(c: &mut Criterion) {
    let fixture = make_fixture(100_000);
    let mut group = c.benchmark_group("bed-len");
    group.throughput(Throughput::Elements(100_000));
    group.bench_function("lengths_100k", |b| {
        b.iter(|| {
            let mut out = Vec::with_capacity(fixture.len() + 100_000 * 8);
            lengths(Cursor::new(fixture.as_str()), &mut out).unwrap();
        });
    });
    group.finish();
}

criterion_group!(benches, bench_lengths);
criterion_main!(benches);
