use std::env;
use std::process;
use std::io::Write;
use std::fs::File;
use std::io::{self};
use std::io::Read;

mod um;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn usage(script_name: &String) {
    println_stderr!("Usage: {} [input binary]", script_name);
}

fn main() {
    let script_name: String = match env::args().nth(0) {
        Some(script_name) => script_name,
        None => panic!("Could not get script name")
    };

    let input_file_name: Option<String> = if env::args().count() != 2 {
        None
    } else {
        env::args().nth(1)
    };

    let input_file_name_str = match input_file_name {
        None => {
            usage(&script_name);
            process::exit(1);
        },
        Some(name) => name
    };

    let mut input_file: File = match File::open(&input_file_name_str) {
        Ok(file) => file,
        Err(_) => panic!("Could not open file {}", &input_file_name_str)
    };
    let mut um: um::UmState = um::UmState::new(&input_file);
    use um::segmented_memory::SegmentedMemory;
    let mut mem = SegmentedMemory::new(&decode_file(&input_file));
    um.run();
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
