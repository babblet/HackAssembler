// Error handling in rust: https://blog.burntsushi.net/rust-error-handling/
// Why rust? https://www.youtube.com/watch?v=cDFSrVhnZKo
// What is an assembler? basically an parser...


// An assembler for the Nand2tetris course that translates assembler to 'simulated bits' a.k.a a string with 1:s and 0:s

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::string::String;
use hack_assembler::assembler;
use hack_assembler::assembler::Code;
use hack_assembler::assembler::Parser;
use hack_assembler::assembler::CommandType;

fn main () {
	let arguments: Vec<String> = env::args().collect();
	let arg1 = &arguments[1];
	let arg2 = &arguments[2];

	let file_path_assembler = Path::new(&arg1);
	let file_path_output = Path::new(&arg2);

	let writable_file: File = match File::open(file_path_assembler){
		Ok(file) => file,
		Err(e)	 => panic!("Error when opening file: {}", e),
	};
	
	let mut parser: Parser = match Parser::new(writable_file) {
		Some(obj) => obj,
		None 	  => panic!("UNEXPECTED ERROR: We was able to read the file but somehow it did not return it!"),
	};

	// Remove unwanted characters from input
	while parser.has_more_commands() {
		parser.advance();
	}
	
	let mut outfile: File = match File::create(file_path_output) {
		Ok(file) => file,
		Err(e) => panic!("Error when creating file: {}", e),
	};

	//Assemble the code
	let mut file_buffer = String::new();	
	for line in parser.lines.iter() {
		//println!("{}", line.buffer);
		if(line.commandType == CommandType::A) {
			file_buffer.push_str(&Code::memo(line.buffer.clone()));
			file_buffer.push('\n');
		} else {
			file_buffer.push_str(&"111".to_string());
			file_buffer.push_str(&Code::comp(line.comp.clone()));
			file_buffer.push_str(&Code::dest(line.dest.clone()));
			file_buffer.push_str(&Code::jump(line.jump.clone()));
			file_buffer.push('\n');
		}
	}
	match outfile.write(file_buffer.as_bytes()) {
		Ok(total_bytes_writen) => println!("Write {} bytes to file", total_bytes_writen),
		Err(e) => panic!("Error when writing to file: {}", e) 
	}
}
