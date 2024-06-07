use std::{fs::File, io::{Read, Write}};
use byteorder::{ByteOrder, LittleEndian};
use num_rational::Rational32;

//ALL IS LITTLE ENDIAN
fn main(){

	let args: Vec<String> = get_args();

	let (sample_vec, _, _) = open_wav_file(args.get(1).unwrap());

	save(encode<i16,16>(sample_vec), args);

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

fn probability_num_denom(in_data: &Vec<i16>) -> (Vec<u64>, u64){

	let mut denom: u64 = 0;

	let mut out_vec: Vec<u64> = [0; 65535].into_iter().collect();

	in_data.iter().for_each(|&e| {

		if *out_vec.get(e as usize).unwrap() == 0{
			denom += 1;
			
			let x = out_vec.get_mut(e as usize).unwrap();
			*x += 1;
		}

	});

	return (Vec::new(), denom);
}

//sequentially encodes byte Vec with arithmetic encoding
fn encode(data: Vec<i16>) -> Vec<u8>{
	
	//TODO

	let (map, denom) = probability_num_denom(&data);

	let mut top = Rational32::new(1, 1);
	let mut bot = Rational32::new(0, 1);


	data.iter().for_each(|e| {



		let diff = top - bot;

	});










	create_arith_header(1, 1, 1)
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

#[allow(unused)]
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