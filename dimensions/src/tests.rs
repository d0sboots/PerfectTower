use crate::JavaRNG;

/// Iterates across all seeds that can be returned from int_range(-0x80000000, 0x7fffffff),
/// which is how the UnityRNG gets seeded. Due to the use of floats internally, only 2^24
/// unique seeds are possible.
///
/// This version uses a slower algorithm that is copied directly from int_range. It is used to
/// validate possible_seeds().
fn possible_seeds_orig() -> impl Iterator<Item = i32> {
    struct Iter {
        x: u32,
    }
    impl Iterator for Iter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.x >= 1 << 24 {
                return None;
            }
            // Duplicate logic of int_range. Most of this will get optimized away.
            const MIN: i32 = -0x80000000;
            const MAX: i32 = 0x7fffffff;
            const FMIN: f64 = MIN as f64;
            const FMAX: f64 = MAX as f64;
            let fresult = (FMAX - FMIN) * f64::from(self.x as f32 * (-24f32).exp2());
            let iresult: i64 = unsafe { fresult.to_int_unchecked() };
            self.x += 1;
            Some((iresult + i64::from(MIN)) as i32)
        }
    }

    Iter { x: 0 }
}

#[test]
fn seeds_align() {
    assert!(possible_seeds_orig().eq(JavaRNG::possible_seeds()));
}
