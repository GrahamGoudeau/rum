use std::rc::Rc;
use std::fs::File;
use std::io::Read;

mod segmented_memory;

const NUM_REGISTERS: usize = 8;
const OPCODE_FIELD_WIDTH: u8 = 4;
const REGISTER_FIELD_WIDTH: u8 = 3;
const THREE_REG_A_LSB: u8 = 6;
const THREE_REG_B_LSB: u8 = THREE_REG_A_LSB - 3;
const THREE_REG_C_LSB: u8 = THREE_REG_B_LSB - 3;
const ONE_REG_A_LSB: u8 = 25;
const ONE_REG_VALUE_LSB: u8 = 0;

struct Three_Register_Data {
    regA: i32,
    regB: i32,
    regC: i32
}

enum Op_Codes {
    Cond_Move(Three_Register_Data),
    Seg_Load(Three_Register_Data)
}

pub struct UmState {
    program_counter: usize,
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

fn cond_move(state: &mut UmState, instruction: Op_Codes) -> usize {
    match instruction {
        Op_Codes::Seg_Load(data) => state.program_counter,
        _ => panic!("Nope")
    }
}

fn seg_load(state: &mut UmState, instruction: Op_Codes) -> usize {
    state.program_counter
}

impl UmState {
    pub fn new(input_code: File) -> UmState {
        let instructions = decode_file(input_code);
        let memory = segmented_memory::SegmentedMemory::new(&instructions);
        UmState {
            program_counter: 0,
            registers: [0; NUM_REGISTERS],
            segmented_memory: memory
        }
    }

    pub fn run(&mut self) {
        let dispatch_table = [
            cond_move,
            seg_load
        ];
        loop {
            let instruction: i32 = self.segmented_memory.fetch_instruction(self.program_counter);
        }
    }
}
