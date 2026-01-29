// import
use std::env;
use std::fs;
use regex::Regex;

// main
fn main(){

	//get file and make manual input possible
	let args: Vec<String> = env::args().collect();
	let input_file = if args.len()>1{&args[1]} else {"input.txt"};
	println!("File_input:{}", input_file);

	//match and use read_to_string()
	match fs::read_to_string(input_file){

		Ok(content) => {
			
			//Regex
			let re = Regex::new(r"\d").unwrap(); 
			let masked = re.replace_all(&content, "*");
			
			//write 
			fs::write("masked_output.txt", masked.to_string()).expect("Failed to precess");
			println!("Process succeeded, see masked_output.txt");}

		Err(_) => {
			println!("Error: Could not find file '{}', Please double check if it exists", input_file);}}}
