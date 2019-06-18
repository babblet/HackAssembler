// Error handling in rust: https://blog.burntsushi.net/rust-error-handling/

use std::env;
use std::fs::File;
use std::io::Write;
use std::string::String;
use hack_assembler::parser;

fn main () {
	//Read assembly file
	let args: Vec<_> = env::args().collect();
	
	let mut buffer = String::new();
	let writable_file = match File::open(&args[1]){
		Ok(()) => println!("File {} was opened", args[1]),
		Err(e) => eprintln!("Error when opening file: {}", e),
	};
	parser::parse(writable_file, &mut buffer);

	let mut outfile = File::create("a.out").unwrap();
	outfile.write(buffer.as_bytes());

	
	//Parser to output in binary to an out file
}
