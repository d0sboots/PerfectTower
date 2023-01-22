use std::num::Wrapping;
use std::str;

// Utility that duplicates The Perfect Tower II's logic for generating
// dimensions, letting you find them more efficiently than clicking around.

// Random number implementation stolen from Java. Used for various operations,
// primarily to seed Unity's RNG.
#[derive(Copy, Clone, Debug)]
pub struct JavaRNG {
    seed: Wrapping<u64>,
}

impl JavaRNG {
    const MASK: Wrapping<u64> = Wrapping(0xFFFFFFFFFFFF);
    const PRIME: Wrapping<u64> = Wrapping(0x5DEECE66D);
    fn new(seed: u64) -> JavaRNG {
        JavaRNG {
            seed: (Wrapping(seed) ^ Self::PRIME) & Self::MASK,
        }
    }

    pub fn from_coords(x: i32, y: i32) -> JavaRNG {
        JavaRNG::new((i64::from(y) * 0x40000000 + i64::from(x)) as u64)
    }

    pub fn next_float(&mut self) -> f32 {
        self.seed = (self.seed * Self::PRIME + Wrapping(0xb)) & Self::MASK;
        (self.seed >> 24).0 as f32 * (-24f32).exp2()
    }

    pub fn int_range(&mut self, min: i32, max: i32) -> i32 {
        let fmin = f64::from(min);
        let fmax = f64::from(max);
        let fresult = ((fmax - fmin) * f64::from(self.next_float()) + fmin).floor();
        unsafe { fresult.to_int_unchecked() }
    }

    pub fn float_range(&mut self, min: f32, max: f32) -> f32 {
        (max - min) * self.next_float() + min
    }
}

// Unity's internal RNG (https://docs.unity3d.com/ScriptReference/Random.html)
// Used for most of the actual random number generation.
#[derive(Copy, Clone, Debug)]
pub struct UnityRNG {
    state: u128,
}

impl UnityRNG {
    pub fn new(seed: i32) -> UnityRNG {
        let mut wrap_seed = Wrapping(seed);
        let mut state = 0u128;
        const PRIME: Wrapping<i32> = Wrapping(0x6c078965);
        for i in 0..4 {
            state |= u128::from(wrap_seed.0 as u32) << (i * 32);
            wrap_seed = wrap_seed * PRIME + Wrapping(1);
        }
        UnityRNG { state }
    }
    pub fn set_seed(&mut self, seed: i32) {
        *self = UnityRNG::new(seed);
    }
    pub fn next_uint(&mut self) -> u32 {
        let mut t = self.state as u32;
        t ^= t << 11;
        t ^= t >> 8;
        let s = (self.state >> 96) as u32;
        self.state >>= 32;
        let result = t ^ s ^ (s >> 19);
        self.state |= u128::from(result) << 96;
        result
    }
    pub fn next_float(&mut self) -> f32 {
        (self.next_uint() & 0x7fffff) as f32 * (-23f32).exp2()
    }
    pub fn int_range(&mut self, min: i32, max: i32) -> i32 {
        if min < max {
            return min.wrapping_add((self.next_uint() % max.wrapping_sub(min) as u32) as i32);
        }
        if max < min {
            return min.wrapping_sub((self.next_uint() % min.wrapping_sub(max) as u32) as i32);
        }
        min
    }
}

pub struct Dimension {
    name: [u8; 9],
    name_size: u8,
}

impl Dimension {
    pub fn new(xcoord: i32, ycoord: i32) -> Dimension {
        let mut dim = Dimension { name: [0; 9], name_size: 0 };
        let mut rng = JavaRNG::from_coords(xcoord, ycoord);
        let mut urng = UnityRNG::new(rng.int_range(-0x80000000, 0x7fffffff));
        dim.name_size = Self::generate_resource_name(&mut dim.name, &mut urng);
        dim
    }

    pub fn name(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.name[0..usize::from(self.name_size)]) }
    }

    fn generate_resource_name(out: &mut [u8; 9], rng: &mut UnityRNG) -> u8 {
        const VOWELS: &[u8; 5] = b"AEIOU";
        const CONSONANTS: &[u8; 21] = b"BCDFGHJKLMNPQRSTVWXYZ";

        let pairs = rng.int_range(0, 3) + 2;
        let mut outidx = 0;
        fn add_char(out: &mut [u8], outidx: &mut usize, rng: &mut UnityRNG, letters: &[u8]) {
            out[*outidx] = letters[rng.int_range(0, letters.len() as i32) as usize];
            *outidx += 1;
        }
        for _ in 0..pairs {
            if rng.next_float() < 0.1 {
                add_char(out, &mut outidx, rng, VOWELS);
                add_char(out, &mut outidx, rng, VOWELS);
            } else {
                if rng.next_float() < 0.5 {
                    add_char(out, &mut outidx, rng, VOWELS);
                    add_char(out, &mut outidx, rng, CONSONANTS);
                } else {
                    add_char(out, &mut outidx, rng, CONSONANTS);
                    add_char(out, &mut outidx, rng, VOWELS);
                }
            }
        }
        if rng.next_float() < 0.5 {
            if VOWELS.contains(&out[usize::from(outidx) - 1]) {
                add_char(out, &mut outidx, rng, CONSONANTS);
            } else {
                add_char(out, &mut outidx, rng, VOWELS);
            }
        }
        outidx as u8
    }
}
