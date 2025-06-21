use std::env;
use std::fs;
use std::process;
use braingraft::compile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input.ll>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let llvm_ir = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            process::exit(1);
        }
    };

    let output = compile(&llvm_ir);
    // The compiled code will be output here in the future
    println!("Compilation result:\n{}", output);
}
