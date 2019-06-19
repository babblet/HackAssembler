pub mod assembler {
	use std::io::Read;
	use std::fs::File;
	use std::string::String;

	// Assembly instuction and its corresponding bit value
	struct Instruction {
		instruction: String,
		bit_value: String,
	}

	//const 

	pub fn translate(buffer: &mut String) {
		let mut tempBuffer: String;
		let mut dest: String;
		let mut comp: String;
		let mut jump: String;
		for(char in buffer) {
			
		}		
	}

	pub fn parse(mut file_to_parse: File, buffer: &mut String) {
		match file_to_parse.read_to_string(buffer) {
			Ok(total_bytes_read) => println!("Read {} bytes from file", total_bytes_read),
			Err(e) => eprintln!("Error while reading from file -> {}", e),
		}
	}
}
