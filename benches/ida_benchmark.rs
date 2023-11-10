use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rabin_ida::RabinIDA;

fn share_creation_benchmark(c: &mut Criterion, (n, k): (u8, u8)) {
    let sharer = RabinIDA {
        shares: n,
        threshold: k,
    };
    let data = vec![3u8; 1024 * 1024];
    //  let rec = sharer.reconstruct(shares[1..=k as usize].to_vec()).unwrap();
    c.bench_function(&format!("ida 1mb {k} of {n} create"), |b| {
        b.iter(|| sharer.share(black_box(data.clone())))
    });

    let mut group = c.benchmark_group("throughput-create-shares");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function(format!("create_shares_1mb: {k} of {n}"), |b| {
        b.iter(|| sharer.share(black_box(data.clone())));
    });
    group.finish();
}

fn reconstruction_benchmark(c: &mut Criterion, (shares, threshold): (u8, u8)) {
    let sharer = RabinIDA { shares, threshold };
    let data = vec![3u8; 1024 * 1024];
    let rabin_shares = sharer.share(data.clone());
    c.bench_function(&format!("ida 1mb {threshold} of {shares} reconstruct"), |b| {
        b.iter(|| {
            sharer
                .reconstruct(black_box(rabin_shares[1..=threshold as usize].to_vec()))
                .unwrap()
        })
    });
    let mut group = c.benchmark_group("throughput-reconstruct-shares");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function(format!("reconstruct_shares_1mb: {threshold} of {shares}"), |b| {
        b.iter(|| {
            sharer
                .reconstruct(black_box(rabin_shares[1..=threshold as usize].to_vec()))
                .unwrap()
        })
    });
    group.finish();
}

fn benchmark_n_k(c: &mut Criterion, scheme: (u8, u8)) {
    share_creation_benchmark(c, scheme);
    reconstruction_benchmark(c, scheme);
}

pub fn ida_benchmark(c: &mut Criterion) {
    benchmark_n_k(c, (7, 5));
    benchmark_n_k(c, (31, 17));
    benchmark_n_k(c, (32, 16));
    benchmark_n_k(c, (50, 40));
}

criterion_group!(benches, ida_benchmark);
criterion_main!(benches);
