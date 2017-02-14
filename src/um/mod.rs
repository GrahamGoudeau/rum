use std::process;
use std::fs::File;
use std::io::{self, Write};
use std::io::Read;

pub mod segmented_memory;

const NUM_REGISTERS: usize = 8;
const OPCODE_FIELD_WIDTH: u8 = 4;
const REGISTER_FIELD_WIDTH: u8 = 3;
const VALUE_FIELD_WIDTH: u8 = 25;
const THREE_REG_A_LSB: u8 = 6;
const THREE_REG_B_LSB: u8 = THREE_REG_A_LSB - 3;
const THREE_REG_C_LSB: u8 = THREE_REG_B_LSB - 3;
const ONE_REG_A_LSB: u8 = 25;
const ONE_REG_VALUE_LSB: u8 = 0;

pub struct UmState {
    program_counter: usize,
    registers: [u32; NUM_REGISTERS],
    segmented_memory: segmented_memory::SegmentedMemory,
}

fn decode_file(input_code: &File) -> Vec<u32> {
    let mut code: Vec<u32> = Vec::new();
    let mut counter = 0;
    let mut instruction: u32 = 0;
    for byte_result in input_code.bytes() {
        match byte_result {
            Ok(byte) => {
                instruction |= (byte as u32) << ((4 - (counter + 1)) * 8);
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

fn get_field(word: u32, lsb: u8, width: u8) -> u32 {
    ((word as u32) << (32 - (width + lsb)) >> (32 - width)) as u32
}

fn get_three_reg_a(instruction: u32) -> usize {
    get_field(instruction, THREE_REG_A_LSB, REGISTER_FIELD_WIDTH) as usize
}

fn get_three_reg_c(instruction: u32) -> usize {
    get_field(instruction, THREE_REG_C_LSB, REGISTER_FIELD_WIDTH) as usize
}

fn get_three_reg_b(instruction: u32) -> usize {
    get_field(instruction, THREE_REG_B_LSB, REGISTER_FIELD_WIDTH) as usize
}

fn get_load_reg_a(instruction: u32) -> usize {
    get_field(instruction, ONE_REG_A_LSB, REGISTER_FIELD_WIDTH) as usize
}

fn get_load_value(instruction: u32) -> u32 {
    get_field(instruction, ONE_REG_VALUE_LSB, VALUE_FIELD_WIDTH)
}

fn cond_move(state: &mut UmState, instruction: u32) -> usize {
    if state.registers[get_three_reg_c(instruction)] != 0 {
        state.registers[get_three_reg_a(instruction)] = state.registers[get_three_reg_b(instruction)];
    }
    state.program_counter + 1
}

fn seg_load(state: &mut UmState, instruction: u32) -> usize {
    let memory_result = state.segmented_memory.read_word(state.registers[get_three_reg_b(instruction)] as u32, state.registers[get_three_reg_c(instruction)] as usize);
    state.registers[get_three_reg_a(instruction)] = memory_result;
    state.program_counter + 1
}

fn seg_store(state: &mut UmState, instruction: u32) -> usize {
    state.segmented_memory.write_word(
        state.registers[get_three_reg_a(instruction)] as u32,
        state.registers[get_three_reg_b(instruction)] as usize,
        state.registers[get_three_reg_c(instruction)] as u32);
    state.program_counter + 1
}

fn math<F: Fn(u32, u32) -> u32>(state: &mut UmState, instruction: u32, operator: F) -> usize
where F: Fn(u32, u32) -> u32 {
    let result = operator(
                    state.registers[get_three_reg_b(instruction)],
                    state.registers[get_three_reg_c(instruction)]);
    state.registers[get_three_reg_a(instruction)] = result;
    state.program_counter + 1
}

fn add(state: &mut UmState, instruction: u32) -> usize {
    math(state, instruction, u32::wrapping_add)
}

fn mult(state: &mut UmState, instruction: u32) -> usize {
    math(state, instruction, u32::wrapping_mul)
}

fn div(state: &mut UmState, instruction: u32) -> usize {
    math(state, instruction, u32::wrapping_div)
}

fn nand(state: &mut UmState, instruction: u32) -> usize {
    math(state, instruction, |a, b| !(a & b))
}

fn halt(state: &mut UmState, instruction: u32) -> usize {
    process::exit(0)
}

fn map_segment(state: &mut UmState, instruction: u32) -> usize {
    let size = state.registers[get_three_reg_c(instruction)];
    let new_id = state.segmented_memory.map_new_segment(size as usize);
    state.registers[get_three_reg_b(instruction)] = new_id;
    state.program_counter + 1
}

fn unmap_segment(state: &mut UmState, instruction: u32) -> usize {
    state.segmented_memory.unmap_segment(state.registers[get_three_reg_c(instruction)] as u32);
    state.program_counter + 1
}

fn output(state: &mut UmState, instruction: u32) -> usize {
    print!("{}", state.registers[get_three_reg_c(instruction)] as u8 as char);
    io::stdout().flush().unwrap();
    state.program_counter + 1
}

fn input(state: &mut UmState, instruction: u32) -> usize {
    let input: Option<u32> = io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u32);

    state.registers[get_three_reg_c(instruction)] = match input {
        None => u32::max_value(),
        Some(byte) => byte as u32
    };
    state.program_counter + 1
}

fn load_program(state: &mut UmState, instruction: u32) -> usize {
    let new_segment_zero = state.registers[get_three_reg_b(instruction)];
    state.segmented_memory.load_segment_zero(new_segment_zero as u32);
    state.registers[get_three_reg_c(instruction)] as usize
}

fn load_value(state: &mut UmState, instruction: u32) -> usize {
    state.registers[get_load_reg_a(instruction)] = get_load_value(instruction);
    state.program_counter + 1
}

fn get_opcode_value(instruction: u32) -> u32 {
    (instruction as u32) >> (32 - OPCODE_FIELD_WIDTH)
}

impl UmState {
    pub fn new(input_code: &File) -> UmState {
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
            halt,
            map_segment,
            unmap_segment,
            output,
            input,
            load_program,
            load_value
        ];

        loop {
            let instruction: u32 = self.segmented_memory.fetch_instruction(self.program_counter);
            let opcode_value = get_opcode_value(instruction);
            if opcode_value == 7 {
                break
            };
            self.program_counter = dispatch_table[opcode_value as usize](self, instruction);
        }
    }
}
