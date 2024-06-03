use std::{collections::HashMap, fs::File, io::{Read, Write}, vec};
use num_bigint::ToBigInt;
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

//sequentially encodes byte Vec with arithmetic encoding
fn encode(data: Vec<u16>) -> Vec<u8>{
	
	let mut out_big_rational: BigRational = BigRational::new(13.to_bigint().unwrap(), 1.to_bigint().unwrap());

	let mut upper_bound: BigRational = BigRational::new(1.to_bigint().unwrap(), 1.to_bigint().unwrap());
	let mut lower_bound: BigRational = BigRational::new(0.to_bigint().unwrap(), 1.to_bigint().unwrap());

	//let mut last_upper_bound: BigRational = BigRational::new(1.to_bigint().unwrap(), 1.to_bigint().unwrap());
	//let mut last_lower_bound: BigRational = BigRational::new(0.to_bigint().unwrap(), 1.to_bigint().unwrap());

	let mut size: BigRational = BigRational::new(1.to_bigint().unwrap(), 1.to_bigint().unwrap());

	let mut frequencies: Vec<u64> = vec![0 ; 0xF0000];

	let mut iteration: u64 = 0;

	//add occurences to u16 samples
	for hex in &data{
		let x = frequencies.get_mut(*hex as usize).unwrap();
		*x += 1;
	}

	let freq_sum: u64 = frequencies.clone().into_iter().filter(|x| *x != 0).reduce(|acc, e| acc + e).unwrap();

	dbg!(&freq_sum);
	
	let mut probabilitys: Vec<BigRational> = Vec::new();
	
	for freq in &frequencies{
		probabilitys.push(BigRational::new(freq.clone().to_bigint().unwrap(), freq_sum.to_bigint().unwrap()));
	}

	let mut segments_top: Vec<BigRational> = Vec::new();
	let mut tmp: BigRational = BigRational::new(0.to_bigint().unwrap(), 1.to_bigint().unwrap());
	probabilitys.clone().into_iter().for_each(|e| {
		segments_top.push(e.clone() + tmp.clone()); tmp += e
	}); //DEBUG

	//dbg!(&probabilitys); //DEBUG

	let mut segments_bottom: Vec<BigRational> = Vec::new();
	tmp = BigRational::new(0.to_bigint().unwrap(), 1.to_bigint().unwrap());
	probabilitys.into_iter().for_each(|e| {
		segments_bottom.push(tmp.clone()); tmp += e
	}); //DEBUG


	//iteratively narrow down the span
	for segm in &data[0..data.len()] {

		let seg_t = segments_top.get(*segm as usize).unwrap();
		let seg_b = segments_bottom.get(*segm as usize).unwrap();
		
		upper_bound = &lower_bound + &size * seg_t;
		lower_bound = &lower_bound + &size * seg_b;

		size = upper_bound - lower_bound.clone();
		iteration += 1;
	}

	//TODO: get one bigrational with fewest bits

	let mut data_sizes = (upper_bound.numer().to_bytes_le().1, upper_bound.denom().to_bytes_le().1);

	let mut header: Vec<u8> = create_arith_header(
		data_sizes.0.len() as u64,
		data_sizes.1.len() as u64,
		data.len() as u64
	);

	header.append(&mut data_sizes.0);
	header.append(&mut data_sizes.1);

	header
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