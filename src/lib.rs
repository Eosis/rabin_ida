//! # Sharing

pub mod ida;

mod gf;
mod rabin_share;

#[doc(inline)]
pub use crate::ida::RabinIDA;

#[cfg(test)]
mod tests {
    use crate::ida::RabinIDA;
    use proptest::collection::size_range;
    use rand::{prelude::SliceRandom, thread_rng, Rng};
    use test_strategy::proptest;

    #[proptest]
    fn test_up_to_1_mb(
        #[any(size_range(0..1024).lift())] data: Vec<u8>,
        #[strategy(2..255u8)] n: u8,
        #[strategy(1..#n)] k: u8,
    ) {
        let sharer = RabinIDA::new(n, k);
        let mut shares = sharer.share(data.clone());
        let mut rng = thread_rng();
        shares.shuffle(&mut rng); // test any k shares will recreate data
        let rec = sharer.reconstruct(shares[1..=k as usize].to_vec()).unwrap();
        assert_eq!(data, rec);
    }

    #[test]
    fn test_1_mb() {
        let (n, k) = (10, 7);
        let sharer = RabinIDA::new(n, k);
        let mut rng = thread_rng();
        let data: Vec<_> = (0..1024).map(|_| rng.gen_range(0..=255u8)).collect();
        let mut shares = sharer.share(data.clone());
        shares.shuffle(&mut rng); // test any k shares will recreate data
        let recovered = sharer.reconstruct(shares[1..=k as usize].to_vec()).unwrap();
        assert_eq!(data, recovered);
    }
}
