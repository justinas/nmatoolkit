//! Provides functions to generate random numbers.

use rand::distributions::{Distribution, Uniform};

/// Generates a random integer in the range [min, max).
/// Minimum bound is inclusive, maximum bound is exclusive.
/// E.g. `generate_i64(0, 5)` will return one of:
/// 0, 1, 2, 3, 4.
pub fn generate_i64(min: i64, max: i64) -> i64 {
    let distribution = Uniform::<i64>::new(min, max);
    distribution.sample(&mut rand::thread_rng())
}

mod test {
    #[test]
    fn test_generate_i64() {
        for _ in 0..1000 {
            let result = super::generate_i64(0, 100);
            assert!(result >= 0);
            assert!(result < 100);
        }
    }
}
