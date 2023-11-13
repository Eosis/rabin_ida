//! # Sharing

mod gf;
pub mod ida;
pub mod rabin_share;

pub use crate::rabin_share::RabinShare;

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
        #[strategy(2..255u8)] shares: u8,
        #[strategy(1..#shares)] threshold: u8,
    ) {
        let sharer = RabinIDA { shares, threshold };
        let mut shares = sharer.share(&data);
        let mut rng = thread_rng();
        shares.shuffle(&mut rng); // test any k shares will recreate data
        let rec = sharer.reconstruct(shares[1..=threshold as usize].to_vec()).unwrap();
        assert_eq!(data, rec);
    }

    #[test]
    fn test_1_mb() {
        let threshold = 7;
        let shares = 10;
        let sharer = RabinIDA { shares, threshold };
        let mut rng = thread_rng();
        let data: Vec<_> = (0..1024).map(|_| rng.gen_range(0..=255u8)).collect();
        let mut shares = sharer.share(&data);
        shares.shuffle(&mut rng); // test any k shares will recreate data
        let recovered = sharer.reconstruct(shares[1..=threshold as usize].to_vec()).unwrap();
        assert_eq!(data, recovered);
    }
    #[test]
    fn test_share_coeffs() {
        let shares = 3u8;
        let threshold = 2u8;
        let sharer = RabinIDA { shares, threshold };
        let data: Vec<_> = vec![169, 45, 202, 222];
        let zeroth = sharer.share_at_index(&data, 0);
        assert_eq!(
            data.into_iter().step_by(threshold.into()).collect::<Vec<_>>(),
            zeroth.body
        );
    }

    #[test]
    fn test_share() {
        let data = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        dbg!(RabinIDA { 
            shares: 5,
            threshold: 4
        }.share(&data));
    }
}
