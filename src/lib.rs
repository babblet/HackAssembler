pub mod assembler {
	use std::io::Read;
	use std::fs::File;
	use std::string::String;
	use std::str::Lines;
	use std::str::Chars;
	use std::vec::Vec;

	/// Available command types
	#[derive(PartialEq, Eq, Clone)]
	pub enum CommandType {
		NONE,
		A,
		C,
		L,
	}

	pub struct Line {
		pub commandType: CommandType,
		pub buffer: String,
		pub dest: String,
		pub comp: String,
		pub jump: String,
	}

	impl Line {
		pub fn new(mut buffer: String) -> Line {
			Line {
				commandType: CommandType::NONE,
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
		pub fn advance(&mut self) {
			if !self.has_more_commands || self.next_line_position > (self.lines.len() - 1) {
				self.has_more_commands = false;
				return;
			}

			//Remove whitespace and comments
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

			if (!self.has_more_commands) || ( self.next_line_position < 1) { return; }

			//Get CommandType and slice command to dest, comp and jump
			let current_command: Vec<char> = self.lines[self.next_line_position - 1].buffer.chars().collect();
			if current_command.len() > 0 {
				if current_command[0] == '(' { 
					self.lines[self.next_line_position - 1].commandType = CommandType::L
				}
				else if current_command[0] == '@' { 
					self.lines[self.next_line_position - 1].commandType  = CommandType::A
				}
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
					self.lines[self.next_line_position - 1].commandType = CommandType::C;
				}
			}
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
		pub fn memo(mnemonic: String) -> String {
			println!("Formating: {}", mnemonic);
			let (_, formated) = mnemonic.split_at(1);
			return format!("{:b}", mnemonic.parse::<u16>().unwrap());
		}

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
			if      mnemonic == "0"   { "0101010".to_string() }
			else if mnemonic == "1"   { "0111111".to_string() }
			else if mnemonic == "-1"  { "0111010".to_string() }
			else if mnemonic == "D"   { "0001100".to_string() }
			else if mnemonic == "A"   { "0110000".to_string() }
			else if mnemonic == "!D"  { "0001101".to_string() }
			else if mnemonic == "!A"  { "0110001".to_string() }
			else if mnemonic == "-D"  { "0001111".to_string() }
			else if mnemonic == "-A"  { "0110011".to_string() }
			else if mnemonic == "D+1" { "0011111".to_string() }
			else if mnemonic == "A+1" { "0110111".to_string() }
			else if mnemonic == "D-1" { "0001110".to_string() }
			else if mnemonic == "A-1" { "0110010".to_string() }
			else if mnemonic == "D+A" { "0000010".to_string() }
			else if mnemonic == "D-A" { "0010011".to_string() }
			else if mnemonic == "A-D" { "0000111".to_string() }
			else if mnemonic == "D&A" { "0000000".to_string() }
			else if mnemonic == "D|A" { "0010101".to_string() }
			else if mnemonic == "M"   { "1110000".to_string() }
			else if mnemonic == "!M"  { "1110001".to_string() }
			else if mnemonic == "-M"  { "1110011".to_string() }
			else if mnemonic == "M+1" { "1110111".to_string() }
			else if mnemonic == "M-1" { "1110010".to_string() }
			else if mnemonic == "D+M" { "1000010".to_string() }
			else if mnemonic == "D-M" { "1010011".to_string() }
			else if mnemonic == "M-D" { "1000111".to_string() }
			else if mnemonic == "D&M" { "1000000".to_string() }
			else if mnemonic == "D|M" { "1010101".to_string() }
			else { "".to_string() }
		}
	
		/// Returns the binary code of the jump mnemonic.
		/// Returns: 3 bits (as an String)
		pub fn jump(mnemonic: String) -> String {
			println!("comp: {}", mnemonic);
			if      mnemonic == "JGT" { "001".to_string() }
			else if mnemonic == "JEQ" { "010".to_string() }
			else if mnemonic == "JGE" { "011".to_string() }
			else if mnemonic == "JLT" { "100".to_string() }
			else if mnemonic == "JNE" { "101".to_string() }
			else if mnemonic == "JLE" { "110".to_string() }
			else if mnemonic == "JMP" { "111".to_string() }
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
