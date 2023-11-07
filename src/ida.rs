//! Information Dispersal Algorithms
use crate::gf::{Field, GF};
use crate::rabin_share::RabinShare;

/// # Rabin Information Dispersal
///
/// ```rust
/// use rabin_ida::RabinIDA;
///
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 11, 11, 11, 12, 13, 122].to_vec();
///
/// let n = 255;
/// let k = 100;
/// let sharer = RabinIDA::new(n, k);
///
/// let shares = sharer.share(data.clone());
///
/// // You only need k out of the n shares to reconstruct
/// let rec = sharer.reconstruct(shares[1..=k as usize].to_vec()).unwrap();
///
/// assert_eq!(data, rec);
/// ```
pub struct RabinIDA {
    n: u8,
    k: u8,
}

impl RabinIDA {
    pub fn new(n: u8, k: u8) -> Self {
        Self { n, k }
    }
}

impl RabinIDA {
    pub fn share(&self, data: Vec<u8>) -> Vec<RabinShare> {
        (1..=self.n).map(|x| self.share_at_index(&data, x)).collect()
    }

    /// Return the specific share at the index $i$. The loop is an optimized implementation based on
    /// usage of Vandermonde matrices as the encoding matrix, as well as $x_i$ being set to the index.
    ///
    /// # Arguments
    ///
    /// * `data` - Data to distribute.
    ///
    /// * `index` - Index of the share.
    pub fn share_at_index(&self, data: &Vec<u8>, index: u8) -> RabinShare {
        let gx = GF(index);
        RabinShare {
            id: index,
            original_length: data.len(),
            body: data
                .chunks(self.k as usize)
                .map(|chunk| {
                    chunk
                        .into_iter()
                        .rev()
                        // Optimized Matrix multiplication for Vandermonde encoding matrices
                        .fold(GF::zero(), |res, b| GF(*b) + gx * res)
                        .into()
                })
                .collect(),
        }
    }

    pub fn reconstruct(&self, shares: Vec<RabinShare>) -> Option<Vec<u8>> {
        if shares.len() < self.k as usize {
            return None;
        }
        let xvalues = shares.iter().map(|x| x.id).collect();
        let decoder = generate_decoder(self.k as usize, xvalues);
        let mut secret = vec![0u8; shares[0].original_length];
        for i in 0..shares[0].body.len() {
            for j in 0..self.k as usize {
                let index = (i * self.k as usize) + j;
                if index >= shares[0].original_length {
                    continue;
                }
                secret[index] = (0..self.k as usize)
                    .map(|x| GF(decoder[j][x]) * GF(shares[x].body[i]))
                    .sum::<GF<u8>>()
                    .into();
            }
        }
        Some(secret)
    }
}

fn generate_decoder(size: usize, values: Vec<u8>) -> Vec<Vec<u8>> {
    dbg!(inverse(
        (0..size)
            .map(|i| (0..size).map(|j| GF(values[i]).pow(j).into()).collect())
            .collect(),
    ))
}

fn two_mut<T>(sl: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    let (smaller, larger) = if i < j { (i, j) } else { (j, i) };
    let (smsl, lgsl) = sl.split_at_mut(larger);
    if i == smaller {
        (&mut smsl[smaller], &mut lgsl[0])
    } else {
        (&mut lgsl[0], &mut smsl[smaller])
    }
}

fn inverse(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let size = matrix.len();
    let mut res = generate_identity(size);
    let mut tmp = matrix.clone();

    for i in 0..size {
        let inv = GF(tmp[i][i]).inverse().into();
        normalize_row(&mut tmp[i][..], &mut res[i][..], inv);

        for j in 0..size {
            if j == i {
                continue;
            }
            let coeff = tmp[j][i];
            if coeff == 0 {
                continue;
            }

            let (tmpi, tmpj) = two_mut(&mut tmp[..], i, j);
            let (resi, resj) = two_mut(&mut res[..], i, j);
            mult_and_subtract(&mut tmpj[..], &mut tmpi[..], coeff);
            mult_and_subtract(&mut resj[..], &mut resi[..], coeff);
        }
    }

    return res;
}

fn mult_and_subtract(row: &mut [u8], normalized: &[u8], coeff: u8) {
    for i in 0..row.len() {
        row[i] = (GF(row[i]) - GF(normalized[i]) * GF(coeff)).into();
    }
}

fn normalize_row(tmp_row: &mut [u8], res_row: &mut [u8], element: u8) {
    for i in 0..tmp_row.len() {
        tmp_row[i] = (GF(tmp_row[i]) * GF(element)).into();
        res_row[i] = (GF(res_row[i]) * GF(element)).into();
    }
}

fn generate_identity(size: usize) -> Vec<Vec<u8>> {
    (0..size)
        .map(|i| (0..size).map(|j| if i == j { 1 } else { 0 }).collect())
        .collect()
}
