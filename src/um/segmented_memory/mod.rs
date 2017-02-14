use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
pub enum MemoryError {
    unrecognized_segment_id,
    invalid_segment_size,
    out_of_range
}

pub type MemoryResult<T> = Result<T, MemoryError>;
pub type Segment = Vec<u32>;

pub struct SegmentedMemory {
    segments: Vec<Segment>,
    recycledIds: Vec<u32>,
    maxUnusedId: u32,
}

impl SegmentedMemory {
    pub fn new(initial_instructions: &Vec<u32>) -> SegmentedMemory {
        let seg_zero = initial_instructions.to_owned();
        let mut mem = SegmentedMemory {
            segments: Vec::new(),
            recycledIds: Vec::new(),
            maxUnusedId: 1,
        };
        mem.segments.push(seg_zero);
        mem
    }

    pub fn fetch_instruction(&self, program_counter: usize) -> u32 {
        self.segments[0][program_counter]
    }

    pub fn load_segment_zero(&mut self, segment_id: u32) -> () {
        if segment_id != 0 {
            self.segments[0] = self.segments[segment_id as usize].to_owned();
        }
    }

    pub fn map_new_segment(&mut self, length: usize) -> u32 {
        let new_id = match self.recycledIds.pop() {
            None => {
                let ret = self.maxUnusedId;
                self.maxUnusedId += 1;
                ret
            },
            Some(id) => id
        };
        let new_segment = vec![0; length];
        let segments_len = self.segments.len();
        if new_id as usize >= segments_len {
            self.segments.push(new_segment)
        } else {
            self.segments[new_id as usize] = new_segment
        }
        new_id
    }

    pub fn unmap_segment(&mut self, segment_id: u32) -> u32 {
        self.recycledIds.push(segment_id);
        segment_id
    }

    pub fn write_word(&mut self, segment_id: u32, offset: usize, word: u32) -> u32 {
        self.segments[segment_id as usize][offset as usize] = word;
        segment_id
    }

    pub fn read_word(&self, segment_id: u32, offset: usize) -> u32 {
        self.segments[segment_id as usize][offset as usize]
    }
}
