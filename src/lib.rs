pub mod parser {
	use std::io::Read;
	use std::fs::File;
	use std::string::String;

	pub fn parse(mut file_to_parse: File, buffer: &mut String) {
		match file_to_parse.read_to_string(buffer) {
			Ok(total_bytes_read) => println!("Read {} bytes from file", total_bytes_read),
			Err(e) => eprintln!("Error while reading from file -> {}", e),
		}
	}
}
pub mod code {
	use std::string::String;

	// Assembly instuction and its corresponding bit valueo
	struct Instruction {
		instruction: String,
		bit_value: String,
	}

	pub fn translate(buffer: &mut String) {
		let mut tempBuffer: String;
		let mut dest: String;
		let mut comp: String;
		let mut jump: String;
		let mut counter: u8 = 0;
		for char in buffer.chars() {
			print!("{}", char);
			if char == '\n' {
				counter = counter + 1;
			}
		}
		print!("{}",counter);
	}

	fn dest() {}
}


pub mod SymbolTable {
	fn addEntry() {}
	fn contains() {}
	fn getAddress() {}
}
