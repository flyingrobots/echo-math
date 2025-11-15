//! Integration tests for the echo-math crate.

// These tests are extracted from the original `prng.rs` module.
// They should fail to compile until the `Prng` implementation is moved.

#[cfg(test)]
mod prng_tests {
    use echo_math::Prng;

    #[test]
    fn next_int_returns_single_value_for_equal_bounds() {
        let mut prng = Prng::from_seed(42, 99);
        assert_eq!(prng.next_int(7, 7), 7);
    }

    #[test]
    fn next_int_deterministic_across_calls() {
        let mut a = Prng::from_seed(123, 456);
        let mut b = Prng::from_seed(123, 456);
        for _ in 0..100 {
            assert_eq!(a.next_int(-10, 10), b.next_int(-10, 10));
        }
    }

    #[test]
    fn next_int_respects_bounds() {
        let mut prng = Prng::from_seed(42, 99);
        for _ in 0..1_000 {
            let v = prng.next_int(-10, 10);
            assert!((-10..=10).contains(&v));
        }
        for _ in 0..1_000 {
            let v = prng.next_int(i32::MIN, i32::MAX);
            // All i32 are valid; this simply exercises the path.
            let _ = v;
        }
    }

    // This test is conditionally compiled and will only run if the `golden_prng`
    // feature is enabled in echo-math's Cargo.toml.
    #[cfg(feature = "golden_prng")]
    #[test]
    fn next_int_golden_regression() {
        let mut prng = Prng::from_seed(0xDEAD_BEEF, 0xFACE_FEED);
        let values: Vec<i32> = (0..3).map(|_| prng.next_int(i32::MIN, i32::MAX)).collect();
        assert_eq!(values, vec![1_501_347_292, 1_946_982_111, -117_316_573]);
    }
}
