pub mod assembler {
	use std::io::Read;
	use std::fs::File;
	use std::string::String;
	use std::str::Lines;
	use std::str::Chars;
	use std::vec::Vec;

	/// Available command types
	#[derive(PartialEq, Eq)]
	pub enum CommandType {
		NONE,
		A,
		C,
		L,
	}

	struct Line {
		buffer: String,
		dest: String,
		comp: String,
		jump: String,
	}

	impl Line {
		pub fn new(mut buffer: String) -> Line {
			Line {
				buffer: buffer,
			}
		}
	}

	/// Unpacks each instruction into its underlying fields	
	pub struct Parser {
		buffer: String,
		buffer_size: usize,
		lines: Vec<Line>,
		line_commands: Vec<CommandType>,
		next_line_position: usize,
		has_more_commands: bool,
	}

	impl Parser {

		fn parse_current_line(&mut self) {
		}
	
		/// (Constructor) Opens the input file/stream and gets ready to parse it.
		pub fn new(mut file_to_read: File) -> Option<Parser> {	
			let mut buffer: String = String::new();
			match file_to_read.read_to_string(&mut buffer) {
				Ok(total_bytes_read) => {
					println!("Read {} bytes from file", total_bytes_read);	
					let mut lines: Vec<Line> = Vec::new();
					buffer.lines().for_each( |item| lines.push(Line::new(item.to_string())));
					Some(Parser { 
						buffer: buffer,
						buffer_size: total_bytes_read,
						lines: lines,
						line_commands: Vec::new(),
						next_line_position: 0,
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
		/// Initially there is no current commands
		pub fn advance(&mut self) {
			if !self.has_more_commands || self.next_line_position > (self.lines.len() - 1) {
				self.has_more_commands = false;
				return;
			}


			self.parse_line();
			let mut new_line: String = String::new();
			let mut could_be_a_comment: bool = false;
			for item in self.lines[self.next_line_position].buffer.chars() {
				if item == '/' {
					if could_be_a_comment { break; }
					could_be_a_comment = true;
				}
				else if item != ' ' { new_line.push(item); }
			}

			//Check parsed line
			if new_line.len() == 0 {
				self.lines.remove(self.next_line_position);
			} else {
				println!("{}: {}", self.next_line_position, new_line);
				self.lines[self.next_line_position].buffer = new_line;
				self.next_line_position = self.next_line_position + 1;
			}
		}
	
		/// Returns the type of the current command:
		/// A_COMMAND for @xxx where xxx is either a symbol or a decimal number
		/// C_COMMAND for dest = comp; jump
		/// L_COMMAND for (xxx) where xxx is a symbol.
		pub fn command_type(&mut self) -> CommandType {
			if (!self.has_more_commands) || ( self.next_line_position < 1) { return CommandType::NONE }

			let current_command: Vec<char> = self.lines[self.next_line_position - 1].buffer.chars().collect();
			if current_command.len() > 0 {
				if current_command[0] == '(' { return CommandType::L }
				else if current_command[0] == '@' { return CommandType::A }
				else if current_command[0] != '/' && current_command[0] != ' ' { return CommandType::C }
			}
			CommandType::NONE
		}
	
		///// Returns the dest mnemonic in the current C-command (8 possibilites).
		///// Should be called only when commandType() is C_COMMAND.
		//fn symbol() -> String {
		//	if !(command_type() == CommandType::C) { return "not a c command" }
		//	"..."	
		//}
 	}

	/// Translates each field into its corresponding binary value
	struct Code {

	}

	impl Code {
		/// Returns the binary code of the dest mnemonic.
		/// Returns: 3 bits (as an String)
		fn dest(mnemonic: String) -> String {
			let chars: Vec<char> = mnemonic.chars().collect();
			let code: String = String::new();
			code.push(chars[10]);
			code.push(chars[11]);
			code.push(chars[12]);
			if code == ""    { String::new("000") }
			if code == "M"   { String::new("001") }
			if code == "D"   { String::new("010") }
			if code == "MD"  { String::new("011") }
			if code == "A"   { String::new("100") }
			if code == "AM"  { String::new("101") }
			if code == "AD"  { String::new("110") }
			if code == "AMD" { String::new("111") }
		}
	
		/// Returns the binary code of the comp mnemonic.
		/// Returns: 7 bits (as an String)
		fn comp(mnemonic: String) -> String {
			let chars: Vec<char> = mnemonic.chars().collect();
			let code: String = String::new();
			code.push(chars[4]);
			code.push(chars[5]);
			code.push(chars[6]);
			code.push(chars[7]);
			code.push(chars[8]);
			code.push(chars[9]);
			if code == ""    { String::new("000000") }
			
		}
	
		/// Returns the binary code of the jump mnemonic.
		/// Returns: 3 bits (as an String)
		fn jump(mnemonic: String) -> String {

		}
	}
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
