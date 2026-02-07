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
			let re = Regex::new(r"(\d{2}\.\d{2}\.\d{4}|\b\d{5,}\b)").unwrap(); 
			let masked = re.replace_all(&content, "*");

			//Print messages, also when there is  no content need to be masked
			if masked == content{
				println!("Notice: No sensitive digits found. The output file remains unchanged.");}
			else{
				println!("Detected sensitive PII. Applying masks...")}
			
			//write 
			fs::write("masked_output.txt", masked.to_string()).expect("Failed to precess");
			println!("Process succeeded, see masked_output.txt");}

		Err(_) => {
			println!("Error: Could not find file '{}', Please double check if it exists", input_file);}}}
