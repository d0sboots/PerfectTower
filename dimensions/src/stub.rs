use super::JavaRNG;
use arrayvec::ArrayVec;

/// Represents the same thing as super::Dimension, but without generating all the fields.
/// Instead they are left in "stub" form: Just the RNG seeds that allow them to be tested against
/// precomputed tables of "allowed" names and resources.
#[derive(Default, Debug)]
pub struct Dimension {
    pub x: i32,
    pub y: i32,
    pub name: u32,
    pub stacks: ArrayVec<DimensionResource, 3>,
}

#[derive(Default, Debug)]
pub struct DimensionResource {
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
        // Fill out uninitialized memory instead of copying the struct
        unsafe {
            dim.stacks.set_len(rng.int_range(1, 4) as usize);
        }
        for stack in &mut dim.stacks {
            stack.seed = rng.next_uint();
            stack.qty = rng.next_uint();
        }
        dim
    }
}
