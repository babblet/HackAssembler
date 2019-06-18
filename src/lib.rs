pub mod parser {
	use std::io::Read;
	use std::io::Error;
	use std::fs::File;
	use std::result::Result;
	use std::string::String;

	pub fn parse(mut file_to_parse: File, buffer: &mut String) {
		match file_to_parse.read_to_string(buffer) {
			Ok(total_bytes_read) => println!("Read {} bytes from file", total_bytes_read),
			Err(e) => eprintln!("Error while reading from file -> {}", e),
		}
	}
}
