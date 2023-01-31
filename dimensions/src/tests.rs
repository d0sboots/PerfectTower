use crate::{int_to_qty, qty_to_int, JavaRNG};

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

#[test]
fn int_qty_int() {
    let mut fails = 0;
    const MAX: f32 = 8.5;
    const MIN: f32 = 0.001;
    const SIZE: f32 = MAX - MIN;
    let shift = (-24f32).exp2();
    for i in 000000..(1 << 24) {
        let qty = int_to_qty(i);
        let res = int_to_qty(qty_to_int(qty.into()));
        if qty != res || fails > 0 {
            let mqty = (SIZE * shift) * (i as f32);
            //let fsqueezed: f64 = f64::from(qty) - f64::from(MIN);
            println!(
                "{:.8} != {:.8}, {}, 0x{:08x} 0x{:08x} 0x{:016x}",
                qty,
                res,
                i,
                qty.to_bits(),
                mqty.to_bits(),
                (f64::from(SIZE * shift) * f64::from(i)).to_bits(),
            );
            fails += 1;
        }
        if fails > 20 {
            panic!("Too many fails");
        }
    }
    assert_eq!(fails, 0);
}
