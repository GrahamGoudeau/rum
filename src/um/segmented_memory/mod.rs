use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
pub enum MemoryError {
    unrecognized_segment_id,
    invalid_segment_size,
    out_of_range
}

pub type MemoryResult<T> = Result<T, MemoryError>;
pub type Segment = Vec<i32>;

pub struct SegmentedMemory {
    segments: HashMap<i32, Segment>,
    segment_zero: Segment,
    recycledIds: Vec<i32>,
    maxUnusedId: i32,
}

impl SegmentedMemory {
    pub fn new(initial_instructions: &Vec<i32>) -> SegmentedMemory {
        SegmentedMemory {
            segments: HashMap::new(),
            segment_zero: initial_instructions.to_owned(),
            recycledIds: Vec::new(),
            maxUnusedId: 0,
        }
    }

    pub fn fetch_instruction(&self, program_counter: usize) -> i32 {
        self.segment_zero[program_counter]
    }

    pub fn load_segment_zero(&mut self, segment_id: i32) -> MemoryResult<()> {
        if segment_id == 0 {
            return Ok(())
        }
        match self.segments.get(&segment_id) {
            Some(segment) => {
                self.segment_zero = segment.to_owned();
                Ok(())
            }
            None => Err(MemoryError::unrecognized_segment_id)
        }
    }

    pub fn map_new_segment(&mut self, length: usize) -> MemoryResult<i32> {
        if length < 1 {
            Err(MemoryError::invalid_segment_size)
        } else {
            let new_id = match self.recycledIds.pop() {
                None => {
                    let ret = self.maxUnusedId;
                    self.maxUnusedId += 1;
                    ret
                },
                Some(id) => id
            };
            let new_segment = vec![0; length];
            self.segments.insert(new_id, new_segment);
            Ok(new_id)
        }
    }

    pub fn unmap_segment(&mut self, segment_id: i32) -> MemoryResult<i32> {
        match self.segments.remove(&segment_id) {
            Some(k) => {
                self.recycledIds.push(segment_id);
                Ok(segment_id)
            }
            None => Err(MemoryError::unrecognized_segment_id)
        }
    }

    pub fn write_word(&mut self, segment_id: i32, offset: usize, word: i32) -> MemoryResult<i32> {
        if let Some(segment) = self.segments.get_mut(&segment_id) {
            segment[offset] = word;
            Ok(segment_id)
        } else {
            Err(MemoryError::unrecognized_segment_id)
        }
    }

    pub fn read_word(&self, segment_id: i32, offset: usize) -> MemoryResult<i32> {
        match self.segments.get(&segment_id) {
            Some(segment) => Ok(segment[offset]),
            None => Err(MemoryError::unrecognized_segment_id)
        }
    }
}
