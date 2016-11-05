use std::env;
use std::process;
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

    let input_file: Option<String> = if env::args().count() != 2 {
        None
    } else {
        env::args().nth(1)
    };

    if input_file == None {
        usage(&script_name);
        process::exit(1);
    }

    let mut um: um::UmState = um::UmState::new();
//    println_stderr!("New segment: {}", um.map_new_segment(3));
//    println_stderr!("New segment: {}", um.map_new_segment(1));
//    println_stderr!("New segment: {}", um.map_new_segment(60));
}
