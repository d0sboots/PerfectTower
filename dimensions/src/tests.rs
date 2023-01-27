use crate::JavaRNG;

/// Translates a "seed" value returned from next_uint() into the value that would be returned
/// from int_range(-0x80000000, 0x7fffffff), which is how the UnityRNG gets seeded. Due to the
/// use of floats internally, only 2^24 unique seeds are possible.
///
/// This version uses a slower algorithm that is copied directly from int_range. It is used to
/// validate translate_seed().
fn translate_seed_orig(seed: u32) -> i32 {
    // Duplicate logic of int_range. Most of this will get optimized away.
    const MIN: i32 = -0x80000000;
    const MAX: i32 = 0x7fffffff;
    const FMIN: f64 = MIN as f64;
    const FMAX: f64 = MAX as f64;
    let fresult = (FMAX - FMIN) * f64::from(seed as f32 * (-24f32).exp2());
    let iresult: i64 = unsafe { fresult.to_int_unchecked() };
    (iresult + i64::from(MIN)) as i32
}

#[test]
fn seeds_align() {
    for i in 0..(1 << 24) {
        assert_eq!(translate_seed_orig(i), JavaRNG::translate_seed(i), "{}", i);
    }
}
