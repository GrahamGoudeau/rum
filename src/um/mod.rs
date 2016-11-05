use std::rc::Rc;

mod segmented_memory;

const NUM_REGISTERS: usize = 8;

pub struct UmState {
    program_counter: i32,
    registers: [i32; NUM_REGISTERS],
    segmented_memory: segmented_memory::SegmentedMemory,
    //segment_zero: segmented_memory::Segment
}

impl UmState {
    pub fn new() -> UmState {
        let mut memory = segmented_memory::SegmentedMemory::new();
        let segment_zero = memory.map_new_segment(0);
        let segment_zero1 = memory.map_new_segment(0);
        let segment_zero2 = memory.map_new_segment(0);
        let y = memory.get_segment(0);
        UmState {
            program_counter: 0,
            registers: [0; NUM_REGISTERS],
            segmented_memory: memory
        }
    }
}
