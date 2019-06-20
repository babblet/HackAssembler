// Error handling in rust: https://blog.burntsushi.net/rust-error-handling/
// Why rust? https://www.youtube.com/watch?v=cDFSrVhnZKo
// What is an assembler? basically an parser...


// An assembler for the Nand2tetris course that translates assembler to 'simulated bits' a.k.a a string with 1:s and 0:s

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::string::String;
use hack_assembler::code;
use hack_assembler::parser;

fn main () {
	//Get arguments
	let arguments: Vec<String> = env::args().collect();
	let arg1 = &arguments[1];
	let arg2 = &arguments[2];

	//Translate arguments to Paths
	let file_path_assembler = Path::new(&arg1);
	let file_path_output = Path::new(&arg2);

	//Read assembler file
	let mut file_buffer = String::new();
	let writable_file: File = match File::open(file_path_assembler){
		Ok(file) => file,
		Err(e) => panic!("Error when opening file: {}", e),
	};

	//Parse the content of the assembly file so that it only contains assembly instructions
	parser::parse(writable_file, &mut file_buffer);

	//Translate the assembler instructions to bits
	code::translate(&mut file_buffer);

	//Create an outfile and output buffer to it
	let mut outfile: File = match File::create(file_path_output) {
		Ok(file) => file,
		Err(e) => panic!("Error when creating file: {}", e),
	};
	match outfile.write(file_buffer.as_bytes()) {
		Ok(total_bytes_writen) => println!("Wrote {} bytes to file", total_bytes_writen),
		Err(e) => panic!("Error when writing to file: {}", e) 
	}
}
