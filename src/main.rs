use std::env;
use std::process;
use std::fs::File;
use std::io::Write;

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
    let mut um: um::UmState = um::UmState::new(input_file);
    um.run();
}
