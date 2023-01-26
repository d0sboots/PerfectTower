use arrayvec::ArrayVec;
use serde::ser::{SerializeMap, SerializeStruct, Serializer};
use serde::Serialize;
use std::io::{self, Write};
use std::num::Wrapping;
use std::{fmt, str};

// Utility that duplicates The Perfect Tower II's logic for generating
// dimensions, letting you find them more efficiently than clicking around.

#[cfg(test)]
mod tests;

// Random number implementation stolen from Java. Used for various operations,
// primarily to seed Unity's RNG.
#[derive(Copy, Clone, Debug)]
pub struct JavaRNG {
    seed: Wrapping<u64>,
}

impl JavaRNG {
    const MASK: Wrapping<u64> = Wrapping(0xFFFFFFFFFFFF);
    const PRIME: Wrapping<u64> = Wrapping(0x5DEECE66D);
    fn new(seed: u64) -> Self {
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
        let fmin: f64 = min.into();
        let fmax: f64 = max.into();
        // Optimization/order-of-ops: We moved the addition of min outside the floating-point op,
        // so that the interior result would be non-negative and we could trunc instead
        // of needing floor(). But we need to do the conversion as i64, since the float result can
        // have the range of a u32!
        // This *should* be equivalent, since it should be impossible for our f32 source to
        // generate numbers that would lead to boundary cases with f64 math.
        let fresult = (fmax - fmin) * f64::from(self.next_float());
        let iresult: i64 = unsafe { fresult.to_int_unchecked() };
        (iresult + i64::from(min)) as i32
    }

    /// Iterates across all seeds that can be returned from int_range(-0x80000000, 0x7fffffff),
    /// which is how the UnityRNG gets seeded. Due to the use of floats internally, only 2^24
    /// unique seeds are possible.
    ///
    /// This uses an optimized algorithm that avoids floating-point math.
    pub fn possible_seeds() -> impl Iterator<Item = i32> {
        struct Iter {
            x: u32,
        }
        impl Iterator for Iter {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                if self.x < 1 << 24 {
                    const MIN: i64 = -0x80000000;
                    const MAX: i64 = 0x7fffffff;
                    let result = ((MAX - MIN) as u64) * u64::from(self.x);
                    self.x += 1;
                    Some(((result >> 24) as u32 ^ MIN as u32) as i32)
                } else {
                    None
                }
            }
        }

        Iter { x: 0 }
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
    pub fn new(seed: i32) -> Self {
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

#[derive(Default, Debug)]
pub struct ResourceName {
    chars: [u8; 9],
    size: u8,
}

impl ResourceName {
    /// self may be uninitialized
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

impl AsRef<str> for ResourceName {
    fn as_ref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.chars[0..self.size.into()]) }
    }
}

impl Serialize for ResourceName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Attribute {
    type_: u8,
    count: u8,
}

#[derive(Default, Debug)]
pub struct DimensionalResource {
    name: ResourceName,
    name_scheme: u8,
    flavor_text_key: u8,
    attributes: ArrayVec<Attribute, 5>,
    qty: f32,
}

impl DimensionalResource {
    pub const ATTRIBUTES: [&'static str; 21] = [
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

    pub const FLAVOR_TEXT: [&'static str; 34] = [
        "",
        "It glows in the dark",
        "It's breathing",
        "It pulsates at a low frequency",
        "It produces a low buzzing sound",
        "It moves when you don't look at it",
        "It gets warm when you touch it",
        "It responds to audiovisual stimuli",
        "It vibrates when it gets close to magnets",
        "The air surrounding it feels colder than usual",
        "Upon touching it you feel wholesome",
        "It appears in your dreams",
        "It has the shape of a mammal",
        "It becomes invisible when you lick it",
        "Its surface is uneven and coarse",
        "It's way heavier than it looks",
        "It slowly pulls nearby objects towards it",
        "It crawls away from magnets",
        "It absorbs light somehow",
        "Voices emit from it when placed in a dark environment",
        "Touching it erases your skin",
        "It knows your friends and your family",
        "It talks to you in your dreams",
        "It stares back at you",
        "It grows arms and legs when not looking at it",
        "Eating it makes your knees bend backwards",
        "It constantly talks about antimatter",
        "Touching it reveals your future",
        "You feel an otherwordly presence when you sit down next to it",
        "It only wants to sit on the top shelf",
        "Attempts to break it results in a thermonuclear explosion.",
        "It has been to France.",
        "Its favourite food is fried seashells.",
        "Rubbing it on your cheeks provides nightmares followed by months of insomnia.",
    ];

    pub const NAME_SCHEME: [&'static str; 6] = [
        "_",
        "Pieces of _",
        "Shards of _",
        "Droplets of _",
        "_ Ingots",
        "_ Ore",
    ];

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn full_name(&self) -> String {
        Self::NAME_SCHEME[usize::from(self.name_scheme)]
            .to_owned()
            .replace('_', self.name())
    }

    /// self may be uninitialized
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
                self.flavor_text_key = (urng.int_range(0, 20) + 1) as u8;
            } else {
                // dimensional.flavor.text.rare.
                self.flavor_text_key = (urng.int_range(0, 13) + 21) as u8;
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
        unsafe {
            self.attributes.set_len(urng.int_range(1, 6) as usize);
        }
        let flen = self.attributes.len() as f64;
        for attr in &mut self.attributes {
            let idx = urng.int_range(0, attribute_types.len() as i32) as usize;
            attr.type_ = attribute_types[idx];
            // We have to shift everything, to properly simulate C# remove_at
            attribute_types.copy_within(idx + 1.., idx);
            let newlen = attribute_types.len() - 1;
            attribute_types = &mut attribute_types[..newlen];

            let val = f64::from(urng.next_float()) * 99.0 / flen;
            // Exploit the rounding mode of floating point math (specified as round-ties-to-even)
            // to round our numbers, since f64::round does the wrong thing.
            // It's safe to fold the addition of 1.0 into BIG, because none of the reachable values
            // can end up rounding to x.5 after the addition of 1.0, unless they were already x.5.
            const BIG: f64 = 1.0 / f64::EPSILON;
            attr.count = unsafe { ((val + (1.0 + BIG)) - BIG).to_int_unchecked() };
        }
    }

    pub fn write_compact<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        let fqty = (f64::from(self.qty) - f64::from(0.001f32))
            * (24f64.exp2() / f64::from(8.5f32 - 0.001f32))
            + 0.5;
        let iqty: u32 = unsafe { fqty.to_int_unchecked() };
        write!(
            writer,
            "{}{}{} {:06x}",
            char::from(self.flavor_text_key + 64),
            self.name_scheme,
            self.name.as_ref(),
            iqty,
        )?;
        for attr in &self.attributes {
            write!(writer, " {}{}", char::from(attr.type_ + 65), attr.count,)?;
        }
        Ok(())
    }
}

impl fmt::Display for DimensionalResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:21} {:.3}/sec", self.full_name(), self.qty)?;
        if self.flavor_text_key != 0 {
            write!(
                f,
                "  \"{}\"",
                Self::FLAVOR_TEXT[usize::from(self.flavor_text_key)]
            )?;
        }
        for attr in &self.attributes {
            write!(
                f,
                "\n  {:2} {:10}  {:5.1}",
                attr.count,
                Self::ATTRIBUTES[usize::from(attr.type_)],
                f64::from(attr.count) * f64::from(self.qty),
            )?;
        }
        Ok(())
    }
}

impl Serialize for DimensionalResource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct AttrHelper<'a> {
            attr: &'a [Attribute],
        }
        impl Serialize for AttrHelper<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_map(Some(self.attr.len()))?;
                for attr in self.attr {
                    state.serialize_entry(
                        DimensionalResource::ATTRIBUTES[usize::from(attr.type_)],
                        &attr.count,
                    )?;
                }
                state.end()
            }
        }

        let mut state = serializer.serialize_struct("DimensionalResource", 4)?;
        state.serialize_field("name", &self.full_name())?;
        if self.flavor_text_key == 0 {
            state.skip_field("flavor_text")?;
        } else {
            state.serialize_field(
                "flavor_text",
                Self::FLAVOR_TEXT[usize::from(self.flavor_text_key)],
            )?;
        }
        state.serialize_field(
            "attributes",
            &AttrHelper {
                attr: &self.attributes,
            },
        )?;
        state.serialize_field("qty", &self.qty)?;
        state.end()
    }
}

#[derive(Default, Debug, Serialize)]
pub struct Dimension {
    x: i32,
    y: i32,
    name: ResourceName,
    stacks: ArrayVec<DimensionalResource, 3>,
}

impl Dimension {
    pub fn new(xcoord: i32, ycoord: i32) -> Self {
        let mut dim = Dimension::default();
        dim.x = xcoord;
        dim.y = ycoord;
        let mut rng = JavaRNG::from_coords(xcoord, ycoord);
        dim.name
            .generate(&mut UnityRNG::new(rng.int_range(-0x80000000, 0x7fffffff)));
        // Fill out uninitialized memory instead of copying the struct
        unsafe {
            dim.stacks.set_len(rng.int_range(1, 4) as usize);
        }
        for stack in &mut dim.stacks {
            stack.generate(rng.int_range(-0x80000000, 0x7fffffff));
            stack.qty = rng.float_range(0.001, 8.5);
        }
        dim
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// A small format that's still human-readable
    pub fn write_compact<T: Write>(&self, writer: &mut T) -> io::Result<()> {
        write!(writer, "{}|{} {}[", self.x, self.y, self.name.as_ref())?;
        let mut first = true;
        for stack in &self.stacks {
            if !first {
                writer.write_all(b",")?;
            }
            first = false;
            stack.write_compact(writer)?;
        }
        writer.write_all(b"]\n")?;
        Ok(())
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{} {}}} {}", self.x, self.y, self.name())?;
        for stack in &self.stacks {
            write!(f, "\n{}", stack)?;
        }
        Ok(())
    }
}
