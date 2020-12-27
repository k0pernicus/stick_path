use std::io;
use std::process;

mod lib;
use lib::solver;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

macro_rules! parse_entries {
    ($line: expr, $type:ident) => {
        $line
            .split_ascii_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>()
    };
}

const WIDTH_LOWER_LIMIT: usize = 3;
const HEIGHT_UPPER_LIMIT: usize = 100;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    if inputs.len() != 2 {
        println!("Error: expected two entries: width, and height");
        process::exit(1);
    }
    let width = parse_input!(inputs[0], usize);
    let height = parse_input!(inputs[1], usize);
    if width <= 3 || height > 100 {
        println!(
            "Error: width cannot be lower than {}, and height cannot be greater than {}",
            WIDTH_LOWER_LIMIT, HEIGHT_UPPER_LIMIT
        );
        process::exit(1);
    }
    let mut stick_path_s: solver::StickPathSolver<String, i32> = solver::StickPathSolver::new();
    for i in 0..height as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_end().to_string();
        if line.len() > width {
            println!(
                "Error: expected input of length {}, got {}",
                width,
                line.len()
            );
            process::exit(1);
        }
        // Part the entries
        if i == 0 {
            // let it fail if user input is incorrect
            let entries = parse_entries!(line, String);
            if let Err(error) = stick_path_s.add_entries(entries) {
                println!("Error: cannot parse the latest input due to {}", error);
                process::exit(1);
            }
            continue;
        }
        // Part the outputs
        if i == height - 1 {
            // let it fail if user input is incorrect
            let outputs = parse_entries!(line, i32);
            if let Err(error) = stick_path_s.add_outputs(outputs) {
                println!("Error: cannot parse the latest input due to {}", error);
                process::exit(1);
            }
            continue;
        }
        if let Err(error) = stick_path_s.add_path(line.as_str()) {
            println!("Error: cannot parse the latest input due to {}", error);
            process::exit(1);
        }
    }
    let all_paths = stick_path_s.get_paths();
    for path in all_paths.iter() {
        println!("{:?}", path);
    }
}
