// import
use std::env;
use std::fs;
use regex::Regex;
use std::path::Path;

// main
fn main(){

	//get file and make manual input possible
	let args: Vec<String> = env::args().collect();
	let input_file = if args.len()>1{&args[1]} else {"input.txt"};

	if !Path::new(input_file).exists() && input_file == "input.txt" {
        let demo_content = "Anmeldung zur Untersuchung\n\
                            Name: Otto Schneider\n\
                            Versicherungsnummer: 0000012345\n\
                            Geburtsdatum: 01.01.1990\n\
                            Adresse: Schwarzwaldstrasse 9, 79098, Freiburg im Breisgau\n\
                            Telefon: +491234123456\n\
                            # ---------------------------------------------------------------------\n\
                            Vital Signs:\n\
                            Heart rate: 72bpm\n\
                            Blood pressure: 120/80 mmHg\n\
                            Body Temperature: 36.5 Celsius";

	fs::write(input_file, demo_content).expect("Failed to create demo file");
        println!("Notice: '{}' not found. A professional demo file has been created for you.", input_file);}
	
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
