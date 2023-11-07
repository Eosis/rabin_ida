use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rabin_ida::RabinIDA;

pub fn ida_benchmark(c: &mut Criterion) {
    let data = vec![3u8; 1024];
    let n = 52;
    let k = 17;
    let sharer = RabinIDA::new(n, k);

    let shares = sharer.share(data.clone());
    //  let rec = sharer.reconstruct(shares[1..=k as usize].to_vec()).unwrap();
    c.bench_function(&format!("ida 1mb {k} of {n} create"), |b| {
        b.iter(|| sharer.share(black_box(data.clone())))
    });
    c.bench_function(&format!("ida 1mb {k} of {n} reconstruct"), |b| {
        b.iter(|| { 
            sharer
                .reconstruct(black_box(shares[1..=k as usize].to_vec()))
                .unwrap()
        })
    });
    { 
        let mut group = c.benchmark_group("throughput-create-shares");
        group.throughput(Throughput::Bytes(data.len() as u64));
        group.bench_function(format!("create_shares_1mb: {k} of {n}"), |b| {
            b.iter(|| sharer.share(black_box(data.clone())))
        });
    }

    let mut group = c.benchmark_group("throughput-reconstruct-shares");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function(format!("reconstruct_shares_1mb: {k} of {n}"), |b| {
        b.iter(|| sharer
            .reconstruct(black_box(shares[1..=k as usize].to_vec()))
            .unwrap())
    });
    group.finish();
}

criterion_group!(benches, ida_benchmark);
criterion_main!(benches);
