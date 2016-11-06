use std::process;
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

fn get_field(word: i32, lsb: u8, width: u8) -> i32 {
    ((word as u32) << (32 - (width + lsb)) >> (32 - width)) as i32
}

fn get_three_reg_a(instruction: i32) -> usize {
    get_field(instruction, THREE_REG_A_LSB, REGISTER_FIELD_WIDTH) as usize
}

fn get_three_reg_c(instruction: i32) -> usize {
    get_field(instruction, THREE_REG_C_LSB, REGISTER_FIELD_WIDTH) as usize
}

fn get_three_reg_b(instruction: i32) -> usize {
    get_field(instruction, THREE_REG_B_LSB, REGISTER_FIELD_WIDTH) as usize
}

fn cond_move(state: &mut UmState, instruction: i32) -> usize {
    if get_three_reg_c(instruction) != 0 {
        state.registers[get_three_reg_a(instruction)] = state.registers[get_three_reg_b(instruction)];
    }
    state.program_counter + 1
}

fn seg_load(state: &mut UmState, instruction: i32) -> usize {
    let memory_result = state.segmented_memory.read_word(get_three_reg_b(instruction) as i32, get_three_reg_c(instruction));
    state.registers[get_three_reg_a(instruction)] = match memory_result {
        Ok(value) => value,
        Err(err) => panic!(err)
    };
    state.program_counter + 1
}

fn seg_store(state: &mut UmState, instruction: i32) -> usize {
    state.segmented_memory.write_word(
        get_three_reg_a(instruction) as i32,
        get_three_reg_b(instruction),
        get_three_reg_c(instruction) as i32);
    state.program_counter + 1
}

fn math<F: Fn(i32, i32) -> i32>(state: &mut UmState, instruction: i32, operator: F) -> usize
where F: Fn(i32, i32) -> i32 {
    let result = operator(
                    state.registers[get_three_reg_b(instruction)],
                    state.registers[get_three_reg_c(instruction)]);
    state.registers[(get_three_reg_a(instruction))] = result;
    state.program_counter + 1
}

fn add(state: &mut UmState, instruction: i32) -> usize {
    math(state, instruction, |a, b| a + b)
}

fn mult(state: &mut UmState, instruction: i32) -> usize {
    math(state, instruction, |a, b| a * b)
}

fn div(state: &mut UmState, instruction: i32) -> usize {
    math(state, instruction, |a, b| a / b)
}

fn nand(state: &mut UmState, instruction: i32) -> usize {
    math(state, instruction, |a, b| !(a & b))
}

fn halt(state: &mut UmState, instruction: i32) -> usize {
    process::exit(0)
}

fn get_opcode_value(instruction: i32) -> u32 {
    (instruction as u32) >> (32 - OPCODE_FIELD_WIDTH)
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
            seg_load,
            seg_store,
            add,
            mult,
            div,
            nand,
            halt
        ];
        loop {
            let instruction: i32 = self.segmented_memory.fetch_instruction(self.program_counter);
            let opcode_value = get_opcode_value(instruction);
            get_field(instruction, THREE_REG_B_LSB, REGISTER_FIELD_WIDTH);
            //dispatch_table[opcode_value as usize](self, instruction);
            break;
        }
    }
}
