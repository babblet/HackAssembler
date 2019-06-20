pub mod assembler {
	use std::io::Read;
	use std::fs::File;
	use std::string::String;

	/// Unpacks each instruction into its underlying fields
	
	pub struct Parser {
		pub buffer: String,
		pub buffer_position: usize,	
		pub buffer_size: usize,
		has_more_commands: bool,
		current_command: String,
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
					Some(Parser { 
						buffer: buffer, 
						buffer_position: 0,
						buffer_size: total_bytes_read,
						has_more_commands: true,
						current_command: String::new(), 
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
		/// Should be called only if hasMoreCommands() is true,
		/// Initially there is no current command.
		fn advance() {
			for()
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
