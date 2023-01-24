use std::fmt;
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
        JavaRNG::new((i64::from(y + 10000000) * 0x40000000 + i64::from(x + 10000000)) as u64)
    }

    pub fn next_float(&mut self) -> f32 {
        self.seed = (self.seed * Self::PRIME + Wrapping(0xb)) & Self::MASK;
        (self.seed >> 24).0 as f32 * (-24f32).exp2()
    }

    pub fn int_range(&mut self, min: i32, max: i32) -> i32 {
        let fmin = f64::from(min);
        let fmax = f64::from(max);
        // Optimization/order-of-ops: We moved the addition of min outside the floating-point op,
        // so that the interior result would be non-negative and we could trunc instead
        // of needing floor().
        // This *should* be equivalent, since it should be impossible for our f32 source to
        // generate numbers that would lead to boundary cases with f64 math.
        let fresult = (fmax - fmin) * f64::from(self.next_float());
        ((unsafe { fresult.to_int_unchecked::<i64>() }) + i64::from(min)) as i32
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

#[derive(Default, Copy, Clone, Debug)]
pub struct ResourceName {
    chars: [u8; 9],
    size: u8,
}

impl ResourceName {
    pub fn to_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.chars[0..usize::from(self.size)]) }
    }

    fn generate(&mut self, rng: &mut UnityRNG) {
        const VOWELS: &[u8; 5] = b"aeiou";
        const CONSONANTS: &[u8; 21] = b"bcdfghjklmnpqrstvwxyz";

        let pairs = rng.int_range(0, 3) + 2;
        let mut offset = b'a' - b'A';
        let out_size = &mut 0;
        let mut add_char =
            |out: &mut ResourceName, out_size: &mut usize, rng: &mut UnityRNG, letters: &[u8]| {
                out.chars[*out_size] =
                    letters[rng.int_range(0, letters.len() as i32) as usize] - offset;
                offset = 0;
                *out_size += 1;
            };
        for _ in 0..pairs {
            if rng.next_float() < 0.1 {
                add_char(self, out_size, rng, VOWELS);
                add_char(self, out_size, rng, VOWELS);
            } else {
                if rng.next_float() < 0.5 {
                    add_char(self, out_size, rng, VOWELS);
                    add_char(self, out_size, rng, CONSONANTS);
                } else {
                    add_char(self, out_size, rng, CONSONANTS);
                    add_char(self, out_size, rng, VOWELS);
                }
            }
        }
        if rng.next_float() < 0.5 {
            if VOWELS.contains(&self.chars[usize::from(*out_size) - 1]) {
                add_char(self, out_size, rng, CONSONANTS);
            } else {
                add_char(self, out_size, rng, VOWELS);
            }
        }
        self.size = *out_size as u8;
    }
}

#[derive(Default, Copy, Clone, Debug)]
#[repr(align(2))]
pub struct Attribute {
    type_: u8,
    count: u8,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct DimensionalResource {
    name: ResourceName,
    name_scheme: u8,
    flavor_text_key: u8,
    attribute_size: u8,
    attributes: [Attribute; 5],
    qty: f32,
}

impl DimensionalResource {
    pub fn name(&self) -> &str {
        self.name.to_str()
    }

    pub fn full_name(&self) -> String {
        match self.name_scheme {
            1 => "Pieces of _",
            2 => "Shards of _",
            3 => "Droplets of _",
            4 => "_ Ingots",
            5 => "_ Ore",
            _ => "_",
        }
        .replace('_', self.name())
    }

    pub fn generate(&mut self, seed: i32) {
        let mut urng = UnityRNG::new(seed);
        // Impl gets color as HSVA, we don't care
        urng.next_uint();
        urng.next_uint();
        urng.next_uint();
        urng.next_uint();
        self.name.generate(&mut urng);
        if 0.5 <= urng.next_float() {
            self.name_scheme = urng.int_range(0, 6) as u8;
        } else {
            self.name_scheme = 0;
        }
        if urng.next_float() <= 0.1 {
            // We impose our own offset scheme here, so everything fits into a u8
            if 0.001 <= urng.next_float() {
                // dimensional.flavor.text.
                self.flavor_text_key = (urng.int_range(0, 20) + 0x20) as u8;
            } else {
                // dimensional.flavor.text.rare.
                self.flavor_text_key = (urng.int_range(0, 13) + 0x40) as u8;
            }
        } else {
            self.flavor_text_key = 0;
        }
        // baseHarvestTime, unused
        urng.next_uint();
        let mut attribute_types_arr = [0u8; 21];
        let mut attribute_types = &mut attribute_types_arr[..];
        for i in 0..attribute_types.len() {
            attribute_types[i] = i as u8;
        }
        self.attribute_size = urng.int_range(1, 6) as u8;
        for i in 0..usize::from(self.attribute_size) {
            let attr = &mut self.attributes[i];
            let idx = urng.int_range(0, attribute_types.len() as i32) as usize;
            attr.type_ = attribute_types[idx];
            // We have to shift everything, to properly simulate C# remove_at
            attribute_types.copy_within(idx + 1.., idx);
            let newlen = attribute_types.len() - 1;
            attribute_types = &mut attribute_types[..newlen];

            let val = f64::from(urng.next_float()) * 99.0 / f64::from(self.attribute_size);
            // Exploit the rounding mode of floating point math (specified as round-ties-to-even)
            // to round our numbers, since f64::round does the wrong thing.
            // It's safe to fold the addition of 1.0 into BIG, because none of the reachable values
            // can end up rounding to x.5 after the addition of 1.0, unless they were already x.5.
            const BIG: f64 = 1.0 / f64::EPSILON;
            attr.count = unsafe { ((val + (1.0 + BIG)) - BIG).to_int_unchecked() };
        }
    }
}

impl fmt::Display for DimensionalResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:21} {:.3}/sec", self.full_name(), self.qty)?;
        if self.flavor_text_key != 0 {
            write!(f, "  \"{}\"", match self.flavor_text_key {
                32 => "It glows in the dark",
                33 => "It's breathing",
                34 => "It pulsates at a low frequency",
                35 => "It produces a low buzzing sound",
                36 => "It moves when you don't look at it",
                37 => "It gets warm when you touch it",
                38 => "It responds to audiovisual stimuli",
                39 => "It vibrates when it gets close to magnets",
                40 => "The air surrounding it feels colder than usual",
                41 => "Upon touching it you feel wholesome",
                42 => "It appears in your dreams",
                43 => "It has the shape of a mammal",
                44 => "It becomes invisible when you lick it",
                45 => "Its surface is uneven and coarse",
                46 => "It's way heavier than it looks",
                47 => "It slowly pulls nearby objects towards it",
                48 => "It crawls away from magnets",
                49 => "It absorbs light somehow",
                50 => "Voices emit from it when placed in a dark environment",
                51 => "Touching it erases your skin",
                64 => "It knows your friends and your family",
                65 => "It talks to you in your dreams",
                66 => "It stares back at you",
                67 => "It grows arms and legs when not looking at it",
                68 => "Eating it makes your knees bend backwards",
                69 => "It constantly talks about antimatter",
                70 => "Touching it reveals your future",
                71 => "You feel an otherwordly presence when you sit down next to it",
                72 => "It only wants to sit on the top shelf",
                73 => "Attempts to break it results in a thermonuclear explosion.",
                74 => "It has been to france.",
                75 => "Its favourite food is fried seashells.",
                76 => "Rubbing it on your cheeks provides nightmares followed by months of insomnia.",
                x => panic!("Invalid flavor_text_key: {}", x)
            })?;
        }
        writeln!(f)?;
        const ATTRIBUTES: [&str; 21] = [
            "Cool",
            "Hard",
            "Sweet",
            "Wet",
            "Herbal",
            "Organic",
            "Spicy",
            "Smelly",
            "Creepy",
            "Metallic",
            "Volatile",
            "Unstable",
            "Rusty",
            "Slimy",
            "Fluffy",
            "Spiky",
            "Strange",
            "Sturdy",
            "Exotic",
            "Artificial",
            "Complex",
        ];
        for i in 0..usize::from(self.attribute_size) {
            writeln!(
                f,
                "  {:2} {:10}  {:5.1}",
                self.attributes[i].count,
                ATTRIBUTES[usize::from(self.attributes[i].type_)],
                f64::from(self.attributes[i].count) * f64::from(self.qty),
            )?;
        }
        Ok(())
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Dimension {
    name: ResourceName,
    stacks_size: u8,
    stacks: [DimensionalResource; 3],
}

impl Dimension {
    pub fn new(xcoord: i32, ycoord: i32) -> Dimension {
        let mut dim = Dimension::default();
        let mut rng = JavaRNG::from_coords(xcoord, ycoord);
        dim.name
            .generate(&mut UnityRNG::new(rng.int_range(-0x80000000, 0x7fffffff)));
        dim.stacks_size = rng.int_range(1, 4) as u8;
        for i in 0..usize::from(dim.stacks_size) {
            let stack = &mut dim.stacks[i];
            stack.generate(rng.int_range(-0x80000000, 0x7fffffff));
            stack.qty = rng.float_range(0.001, 8.5);
        }
        dim
    }

    pub fn name(&self) -> &str {
        self.name.to_str()
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name())?;
        for i in 0..usize::from(self.stacks_size) {
            write!(f, "{}", self.stacks[i])?;
        }
        Ok(())
    }
}
