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

	pub struct Line {
		pub buffer: String,
		pub dest: String,
		pub comp: String,
		pub jump: String,
	}

	impl Line {
		pub fn new(mut buffer: String) -> Line {
			Line {
				buffer: buffer,
				dest: String::new(),
				comp: String::new(),
				jump: String::new(),
			}
		}
	}

	/// unpacks each instruction into its underlying fields	
	pub struct Parser {
		buffer: String,
		buffer_size: usize,
		pub lines: Vec<Line>,
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
		/// Returns the type of the current command:
		/// A_COMMAND for @xxx where xxx is either a symbol or a decimal number
		/// C_COMMAND for dest = comp; jump
		/// L_COMMAND for (xxx) where xxx is a symbol.
		pub fn advance(&mut self) -> CommandType {
			if !self.has_more_commands || self.next_line_position > (self.lines.len() - 1) {
				self.has_more_commands = false;
				return CommandType::NONE;
			}

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

			if (!self.has_more_commands) || ( self.next_line_position < 1) { return CommandType::NONE }

			//Return what command it is or parse the C command
			let current_command: Vec<char> = self.lines[self.next_line_position - 1].buffer.chars().collect();
			if current_command.len() > 0 {
				if current_command[0] == '(' { return CommandType::L }
				else if current_command[0] == '@' { return CommandType::A }
				else if current_command[0] != '/' && current_command[0] != ' ' { 
					
					let mut iter: u8 = 0;
					let mut buffer: String = String::new();
					for item in current_command {
						if item == '=' {
							self.lines[self.next_line_position - 1].dest = buffer; 
							buffer = String::new();
						}
						if item == ';' {
							self.lines[self.next_line_position - 1].comp = buffer;
							buffer = String::new();
						}
						buffer.push(item);
						iter = iter + 1;
					}
					return CommandType::C
				}
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
	pub struct Code {

	}

	impl Code {
		/// Returns the binary code of the dest mnemonic.
		/// Returns: 3 bits (as an String)
		pub fn dest(mnemonic: String) -> String {
			println!("dest: {}", mnemonic);
			if 	mnemonic == "M"   { "001".to_string() }
			else if mnemonic == "D"   { "010".to_string() }
			else if mnemonic == "MD"  { "011".to_string() }
			else if mnemonic == "A"   { "100".to_string() }
			else if mnemonic == "AM"  { "101".to_string() }
			else if mnemonic == "AD"  { "110".to_string() }
			else if mnemonic == "AMD" { "111".to_string() }
			else { return "000".to_string(); }
		}
	
		/// Returns the binary code of the comp mnemonic.
		/// Returns: 7 bits (as an String)
		pub fn comp(mnemonic: String) -> String {
			println!("comp: {}", mnemonic);
			let chars: Vec<char> = mnemonic.chars().collect();
			let a_code: char = chars[0];
			let mut code: String = String::new();
			code.push(chars[4]);
			code.push(chars[5]);
			code.push(chars[6]);
			code.push(chars[7]);
			code.push(chars[8]);
			code.push(chars[9]);
			if a_code == '0' {
				if      code == "0"   { "101010".to_string() }
				else if code == "1"   { "111111".to_string() }
				else if code == "-1"  { "111010".to_string() }
				else if code == "D"   { "001100".to_string() }
				else if code == "A"   { "110000".to_string() }
				else if code == "!D"  { "001101".to_string() }
				else if code == "!A"  { "110001".to_string() }
				else if code == "-D"  { "001111".to_string() }
				else if code == "-A"  { "110011".to_string() }
				else if code == "D+1" { "011111".to_string() }
				else if code == "A+1" { "110111".to_string() }
				else if code == "D-1" { "001110".to_string() }
				else if code == "A-1" { "110010".to_string() }
				else if code == "D+A" { "000010".to_string() }
				else if code == "D-A" { "010011".to_string() }
				else if code == "A-D" { "000111".to_string() }
				else if code == "D&A" { "000000".to_string() }
				else if code == "D|A" { "010101".to_string() }
				else { "".to_string() }

			} else if a_code == '1' {
				if      code == "M"   { "110000".to_string() }
				else if code == "!M"  { "110001".to_string() }
				else if code == "-M"  { "110011".to_string() }
				else if code == "M+1" { "110111".to_string() }
				else if code == "M-1" { "110010".to_string() }
				else if code == "D+M" { "000010".to_string() }
				else if code == "D-M" { "010011".to_string() }
				else if code == "M-D" { "000111".to_string() }
				else if code == "D&M" { "000000".to_string() }
				else if code == "D|M" { "010101".to_string() }
				else { "".to_string() }
			} else { "".to_string() }
		}
	
		/// Returns the binary code of the jump mnemonic.
		/// Returns: 3 bits (as an String)
		pub fn jump(mnemonic: String) -> String {
			println!("comp: {}", mnemonic);
			let chars: Vec<char> = mnemonic.chars().collect();
			let mut code: String = String::new();
			code.push(chars[13]);
			code.push(chars[14]);
			code.push(chars[15]);
			if      code == "JGT" { "001".to_string() }
			else if code == "JEQ" { "010".to_string() }
			else if code == "JGE" { "011".to_string() }
			else if code == "JLT" { "100".to_string() }
			else if code == "JNE" { "101".to_string() }
			else if code == "JLE" { "110".to_string() }
			else if code == "JMP" { "111".to_string() }
			else { "000".to_string() }
		}              
	}                      
//                             
//	/// Manages the SybolT able
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
