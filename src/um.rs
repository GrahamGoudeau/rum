use std::collections::HashMap;
use std::vec::Vec;

const NUM_REGISTERS: usize = 8;

pub struct UmState<'a> {
    program_counter: i32,
    registers: [i32; NUM_REGISTERS],
    segments: HashMap<i32, &'a [i32]>,
    recycledIds: Vec<i32>,
    maxUnusedId: i32
}

impl<'a> UmState<'a> {
    pub fn new() -> UmState<'a> {
        UmState {
            program_counter: 0,
            registers: [0; NUM_REGISTERS],
            segments: HashMap::new(),
            recycledIds: Vec::new(),
            maxUnusedId: 0
        }
    }

    pub fn map_new_segment(&self) -> i32 {
        0
    }
}
