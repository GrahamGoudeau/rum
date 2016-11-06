use std::rc::Rc;
use std::fs::File;
use std::io::Read;

mod segmented_memory;

const NUM_REGISTERS: usize = 8;

pub struct UmState {
    program_counter: i32,
    registers: [i32; NUM_REGISTERS],
    segmented_memory: segmented_memory::SegmentedMemory,
}

fn decode_file(input_code: File) -> Vec<i32> {
    let mut code: Vec<i32> = Vec::new();
    let mut counter = 0;
    let mut instruction: i32 = 0;
    for byte_result in input_code.bytes() {
        match byte_result {
            Ok(byte) => {
                instruction |= (byte as i32) << ((4 - (counter + 1)) * 8);
                if counter % 4 == 3 {
                    counter = 0;
                    code.push(instruction);
                    instruction = 0;
                    continue;
                }
                counter += 1;
            },
            Err(_) => panic!("Malformed input file")
        };
    }

    if counter != 0 {
        panic!("Malformed input file");
    }

    code
}

impl UmState {
    pub fn new(input_code: File) -> UmState {
        let instructions = decode_file(input_code);
        let mut memory = segmented_memory::SegmentedMemory::new(&instructions);
        UmState {
            program_counter: 0,
            registers: [0; NUM_REGISTERS],
            segmented_memory: memory
        }
    }
}
