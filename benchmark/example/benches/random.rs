extern crate rand;

use rand::{prng::chacha::ChaChaRng, Rng, RngCore, SeedableRng};

// Return a random number generator
fn make_rng() -> ChaChaRng {
    let seed: [u8; 32] = [
        253, 250, 8, 63, 78, 171, 183, 196, 45, 86, 159, 168, 184, 196, 209, 26, 191, 112, 250,
        247, 135, 175, 224, 99, 183, 46, 180, 83, 55, 79, 1, 209,
    ];
    let mut rng = ChaChaRng::from_seed(seed);
    for _ in 0..20 {
        rng.next_u32();
    }
    rng
}

// Returns n random numbers in [0..m).
pub fn make_indices(n: usize, m: usize) -> Vec<usize> {
    let mut rng = make_rng();
    (0..n).map(|_| rng.gen_range(0, m)).collect()
}
