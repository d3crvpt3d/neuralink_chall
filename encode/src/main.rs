use std::{fs::File, io::{BufReader, Write}};
use num_bigint::BigUint;

use num_rational::Rational;

//ALL IS LITTLE ENDIAN
fn main(){

	let args: Vec<String> = get_args();

	let mut file_iterator: BufReader<File> = BufReader::new(File::open(args.get(1).unwrap()).expect("cant read File"));

	save(encode(&mut file_iterator), args);

}


fn save(buf: Vec<u8>, args: Vec<String>){

	let mut file = File::create(args.get(2).unwrap()).expect("cant create Output File");

	file.write_all(&buf).expect("cant write to file");
}


fn get_args() -> Vec<String>{

	let args: Vec<String> = std::env::args().collect();

	if args.len() != 3 {
		panic!("Args length should be 3 not {}", args.len());
	}

	args
}

#[allow(unused)]
//sequentially encodes byte Vec with arithmetic encoding
fn encode(f: &mut BufReader<File>) -> Vec<u8>{
	
	let mut out_vec: Vec<u8> = Vec::new();

	//	let mut big_rational = BigRational::new(BigUint::new(vec![1]), BigUint::new(vec![1]));

	//TODO use maybe "string_to_byte_vec"
	f;
	big_rational.reduce();
	//TODO

	let mut header: Vec<u8> = create_arith_header(1, 1);

	header.append(&mut out_vec);

	header
}

#[allow(unused)]
// "00101010" -> 00101010u8
fn string_to_byte_vec(string: &mut String) -> Vec<u8>{

	//pad with zeros
	while string.len() % 8 != 0 {
			string.push('0');
	}
	
	string.as_bytes().chunks(8)
		.map(|chunk| std::str::from_utf8(chunk).unwrap())
		.map(|chunk_str| u8::from_str_radix(chunk_str, 2).unwrap())
		.collect()

}


//returns the header for arithmetic encoding in bits (header_size = 16byte)
fn create_arith_header(numerator_length: u64, denominator_length: u64) -> Vec<u8>{

	[numerator_length.to_le_bytes(), denominator_length.to_le_bytes()].concat()

}
