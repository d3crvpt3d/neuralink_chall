use std::{collections::HashMap, fs::File, io::{BufReader, Read, Write}};
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use byteorder::{ByteOrder, LittleEndian};
use num_rational::BigRational;

//ALL IS LITTLE ENDIAN
fn main(){

	let args: Vec<String> = get_args();

	let (sample_vec, _, _) = open_wav_file(args.get(1).unwrap());

	save(encode(sample_vec), args);

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
fn encode(data: Vec<u16>) -> Vec<u8>{
	
	let mut out_vec: String = String::new();

	let mut frequencies: HashMap<u16, u64> = HashMap::new();

	for hex in &data{
		//check if space is none
		if frequencies.get(hex).is_none() {
			frequencies.insert(*hex, 0);
		}

		*frequencies.get_mut(hex).unwrap() += 1;
	}

	if frequencies.is_empty(){panic!("freq vec empty");} //DEBUG

	let freq_sum: u64 = frequencies.clone().into_values().reduce(|acc, e| acc + e).unwrap();

	let mut sorted_by_key: Vec<(u16, u64)> = frequencies.iter().map(|(x, y)| (*x, *y)).collect();

	sorted_by_key.sort_by_key(|k| k.0);

	
	let mut probabilitys: Vec<BigRational> = Vec::new();

	sorted_by_key.into_iter().for_each(|x| 
		if x.1 != 0{
			probabilitys.push(
				BigRational::new( 1.to_bigint().unwrap(), x.1.to_bigint().unwrap())
			)
		}
	);

	dbg!(&probabilitys); //DEBUG

	let mut segments_top: Vec<BigRational> = Vec::new();
	let mut tmp: BigRational = BigRational::new(0.to_bigint().unwrap(), 1.to_bigint().unwrap());

	probabilitys.clone().into_iter().for_each(|e| {segments_top.push(e.clone() + tmp.clone()); tmp += e; eprintln!("{}", tmp)}); //DEBUG

	let mut segments_bottom: Vec<BigRational> = Vec::new();

	tmp = BigRational::new(0.to_bigint().unwrap(), 1.to_bigint().unwrap());

	probabilitys.into_iter().for_each(|e| {segments_bottom.push(tmp.clone()); eprintln!("{}", tmp); tmp += e}); //DEBUG


	//TODO

	let mut header: Vec<u8> = create_arith_header(1, 1, data.len() as u64);

	header.append(&mut string_to_byte_vec(&mut out_vec));

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
fn create_arith_header(numerator_length: u64, denominator_length: u64, data_length: u64) -> Vec<u8>{

	[numerator_length.to_le_bytes(), denominator_length.to_le_bytes(), data_length.to_le_bytes()].concat()

}

fn open_wav_file(path: &str) -> (Vec<u16>, hound::WavSpec, Header){

	let mut file = hound::WavReader::open(path).expect("hound cant open file");

	let x: Vec<u16> = file.samples::<i16>().map(|x| x.unwrap() as u16).collect();


	//get raw header from file
	let mut header_raw: [u8; 44] = [0; 44];
	std::fs::File::open(path).unwrap().read(&mut header_raw).expect("cant read header of wav file");

	let header = Header{
		size: LittleEndian::read_u32(&header_raw[4..8]),

		format_tag: LittleEndian::read_u16(&header_raw[20..22]),
		channels: LittleEndian::read_u16(&header_raw[22..24]),
		sample_rate: LittleEndian::read_u32(&header_raw[24..28]),
		bytes_second: LittleEndian::read_u32(&header_raw[28..32]),
		block_align: LittleEndian::read_u16(&header_raw[32..34]),
		bits_sample: LittleEndian::read_u16(&header_raw[34..36]),

		data_length: LittleEndian::read_u32(&header_raw[40..44]),
	};
	

	(x, file.spec(), header)
}

struct Header{
	size: u32,
	format_tag: u16,
	channels: u16,
	sample_rate: u32,
	bytes_second: u32,
	block_align: u16,
	bits_sample: u16,
	data_length: u32,
}