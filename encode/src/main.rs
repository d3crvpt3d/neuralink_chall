use std::io::BufReader;

fn main() -> Result<_, Error>{

	let args: Vec<String> = get_args();

	let mut file_iterator: BufReader = BufReader::new(File::open(args.get(1))?).except("cant read File \"{}\"", args.get(1));

	let mut out_buff: String = String::new();

	encode(&mut File, &mut String);

	save(out_buff);

}


fn save(buf: String){
	File::write(args.get(2), buf.to_str()).except("cant write file {}", args.get(2));
}


fn get_args() -> Vec<String>{

	let args = std::env::args().collect();

	if args.len() != 3 {
		panic!("Args length should be 3 not {}", args.len());
	}

	args
}


fn encode(){
	s
}
