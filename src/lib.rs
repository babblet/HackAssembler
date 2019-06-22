pub mod assembler {
	use std::io::Read;
	use std::fs::File;
	use std::string::String;
	use std::str::Lines;
	use std::vec::Vec;

	/// Unpacks each instruction into its underlying fields	
	pub struct Parser {
		buffer: String,
		buffer_size: usize,
		lines: Vec<String>,
		line_position: usize,
		has_more_commands: bool,
	}

	impl Parser {
//		/// Available command types
//		enum CommandType {}
	
		/// (Constructor) Opens the input file/stream and gets ready to parse it.
		pub fn new(mut file_to_read: File) -> Option<Parser> {	
			let mut buffer: String = String::new();
			match file_to_read.read_to_string(&mut buffer) {
				Ok(total_bytes_read) => {
					println!("Read {} bytes from file", total_bytes_read);	
					let mut lines: Vec<String> = Vec::new();
					buffer.lines().for_each( |item| lines.push(item.to_string()));	
					Some(Parser { 
						buffer: buffer,
						buffer_size: total_bytes_read,
						lines: lines, 
						line_position: 0,
						has_more_commands: true,
					})
				}, 
				Err(e) => {
					panic!("Error while reading from file -> {}", e); 
				},
			}
		}
		
		/// Are there more lines in the input?
		pub fn has_more_commands(&self) -> bool {
			self.has_more_commands
		}
		
		/// Reads the next command for the input, and makes it the current command.
		/// Takes care of whitespace, if necessary.
		/// Should be called only if has_more_commands() is true,
		/// Initially there is no current command.
		pub fn advance(&mut self) {
			if !self.has_more_commands || self.line_position > (self.lines.len() - 1) {
				self.has_more_commands = false;
				println!("There are no more commands");
				return;
			}	

			let mut new_line: String = String::new();
			let mut could_be_a_comment: bool = false;
			for item in self.lines[self.line_position].chars() {
				if item == '/' {
					if could_be_a_comment {
						break;
					} else {
						could_be_a_comment = true;
					}
				}
				else if item != ' ' {
					new_line.push(item);
				}
			}
			if new_line.len() == 0 {
				println!("Found a line with no commands");
				self.lines.remove(self.line_position);
			} else {
				println!("{}: {}", self.line_position, new_line);
				self.lines[self.line_position] = new_line;
				self.line_position = self.line_position + 1;
			}
		}
	
//		/// Returns the type of the current command:
//		/// A_COMMAND for @xxx where xxx is either a symbol or a decimal number
//		/// C_COMMAND for dest = comp; jump
//		/// L_COMMAND for (xxx) where xxx is a symbol.
//		fn commandType() -> CommandType {}
//	
//		/// Returns the dest mnemonic in the current C-command (8 possibilites).
//		/// Should be called only when commandType() is C_COMMAND.
//		fn symbol() -> String {}
 	}
//
//	/// Translates each field into its corresponding binary value
//	struct Code {
//
//	}
//
//	impl Code {
//		use std::string::String;
//	
//		/// Assembly instruction and its corresponding bit valueo
//		struct Instruction {
//			instruction: String,
//			bit_value: String,
//		}
//	
//	
//		/// Returns the binary code of the dest mnemonic.
//		/// Returns: 3 bits (as an u8)
//		fn dest(mnemonic: String) -> u8 {}
//	
//		/// Returns the binary code of the comp mnemonic.
//		/// Returns: 7 bits (as an u8)
//		fn comp(mnemonic: String) -> u8 {}
//	
//		/// Returns the binary code of the jump mnemonic.
//		/// Returns: 3 bits (as an u8)
//		fn jump(mnemonic: String) -> u8 {}
//	}
//
//	/// Manages the SybolTable
//	struct SymbolTable {
//	
//	}
//
//	impl SymbolTable {
//	
//		/// (Constructor) Creates a new empty symbol table.
//		fn new() {}
//	
//		/// Adds the pair (symbol, address) to the table.
//		fn addEntry(symbol: String, address: u16) {}
//	
//		/// Does the symbol table contain the given symbol?
//		fn contains(symbol: String) -> bool {}
//	
//		/// Returns the address associated with the symbol.
//		fn getAddress() -> i16 {}
//	}
}
