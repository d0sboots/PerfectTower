use super::JavaRNG;
use crate::int_to_qty;
use tinyvec::ArrayVec;

/// Represents the same thing as super::Dimension, but without generating all the fields.
/// Instead they are left in "stub" form: Just the RNG seeds that allow them to be tested against
/// precomputed tables of "allowed" names and resources.
#[derive(Default, Debug)]
pub struct Dimension {
    pub x: i32,
    pub y: i32,
    pub name: u32,
    pub stacks: ArrayVec<[DimensionalResource; 3]>,
}

#[derive(Default, Debug)]
pub struct DimensionalResource {
    pub seed: u32,
    pub qty: u32,
}

impl Dimension {
    pub fn new(xcoord: i32, ycoord: i32) -> Self {
        let mut dim = Dimension::default();
        dim.x = xcoord;
        dim.y = ycoord;
        let mut rng = JavaRNG::new(xcoord, ycoord);
        dim.name = rng.next_uint();
        dim.stacks.set_len(rng.int_range(1, 4) as usize);
        for stack in &mut dim.stacks {
            stack.seed = rng.next_uint();
            stack.qty = rng.next_uint();
        }
        dim
    }

    pub fn qty_sum(&self) -> f64 {
        // Explicitly unroll loop
        let mut sum = f64::from(int_to_qty(self.stacks[0].qty));
        if self.stacks.len() >= 3 {
            sum += f64::from(int_to_qty(self.stacks[2].qty));
        }
        if self.stacks.len() >= 2 {
            sum += f64::from(int_to_qty(self.stacks[1].qty));
        }
        sum
    }

    pub fn cost(&self) -> f64 {
        // Hardcoded faster pow
        let tmp = self.qty_sum() * 0.125 + 1.0;
        let t2 = tmp * tmp * tmp;
        t2 * t2
    }
}
