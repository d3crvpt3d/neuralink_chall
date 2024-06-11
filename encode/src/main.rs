use std::{fs::File, io::{BufWriter, Read, Write}};
use hashbrown::HashMap;
use num_bigint::BigInt;
use num_rational::BigRational;
use serde::{Serialize, Deserialize};

//ALL IS LITTLE ENDIAN
fn main(){

	let args: Vec<String> = get_args();

	let sample_vec: Vec<i16> = open_wav_file(args.get(1).unwrap());

	let mut buf_write: BufWriter<File> = BufWriter::new(File::create(args.get(2).unwrap()).expect("cant create Output File"));

	if args.len() == 4{
		encode(sample_vec, &mut buf_write, args.get(3).expect("arg 3 is no String"));
	}else {
		encode(sample_vec, &mut buf_write, "table.aet");
	}
}


fn get_args() -> Vec<String>{

	let args: Vec<String> = std::env::args().collect();

	if args.len() < 3 {
		panic!("Args length should be at least 3 not {}", args.len());
	}

	args
}

//read lookup table (I LOVE SERDE AND HASHBROWN)
fn nums_pos_and_denom(path: &str) -> HashMap<u16, Segment>{

	let mut file: File = File::open(path).expect("cant read json file");
  let mut contents: String = String::new();
  file.read_to_string(&mut contents).expect("file is not json");

	let map: HashMap<u16, Segment> = serde_json::from_str(&contents).expect("Failed to deserialize HashMap");

	map
}


//sequentially encodes byte Vec with arithmetic encoding
fn encode<W: Write>(data: Vec<i16>, stream: &mut BufWriter<W>, path: &str){

	let segments: HashMap<u16, Segment> = nums_pos_and_denom(path);

	stream.write_all(&create_arith_header(segments.len() as u64)).expect("cant write header");

	let mut byte_buffer: [u8; 1] = [8]; //stores each not full byte. zB. '1','1','0' => [0,0,0,0,0,0,0,0] -> [1,1,0,0,0,0,0,0]

	let mut remaining_bytes_in_buf: u8 = 0;

	let mut o: BigRational = BigRational::from_integer(BigInt::from(1)); //upper bound
	//let mut s: BigRational = BigRational::from_integer(BigInt::from(1)); //size
	let mut u: BigRational = BigRational::from_integer(BigInt::from(0)); //lower bound

	let one_half: BigRational = BigRational::new(BigInt::from(1), BigInt::from(2));
	let two: BigRational = BigRational::new(BigInt::from(2), BigInt::from(1));

	//NEEDS INTENSIVE TESTING
	data.iter().for_each(|&e| {

		let ee = &(e as u16);

		let curr_upper = BigRational::new(
			BigInt::from(segments.get(ee).unwrap().top),
			BigInt::from(segments.len())
		);

		let curr_lower = BigRational::new(
			BigInt::from(segments.get(ee).unwrap().bottom),
			BigInt::from(segments.len())
		);


		//todo!("calculate new upper/lower bound");
		o = &o * &curr_upper;
		u = &o * &curr_lower;

		//get stream by byte and trim size of rationals
		while (u <= one_half && o <= one_half) || (u >= one_half && o >= one_half){

			if u <= one_half && o <= one_half{

				remaining_bytes_in_buf -= 1;
		
				//scale [0, .5] to [0, 1]
				u = &u * &two;
				o = &o * &two;
			}else if u >= one_half && o >= one_half{

				byte_buffer[0] += 1 << (remaining_bytes_in_buf-1);// put element on right position
				remaining_bytes_in_buf -= 1;
		
				//scale [.5, 1] to [0, 1]
				u = (&u - &one_half) * &two;
				o = (&o - &one_half) * &two;
			}

			//flush byte_buffer[0]
			if remaining_bytes_in_buf == 0{
				stream.write_all(&byte_buffer).expect("cant write a full byte to output");
				remaining_bytes_in_buf = 8;
			}

		}

	});

	stream.flush().unwrap();
}


//returns the header for arithmetic encoding in bits (header_size = 16byte)
fn create_arith_header(samples: u64) -> Vec<u8>{

	[samples.to_le_bytes()].concat()

}

fn open_wav_file(path: &str) -> Vec<i16>{

	let mut file = hound::WavReader::open(path).expect("hound cant open file");

	let x: Vec<i16> = file.samples::<i16>().map(|x| x.unwrap()).collect();

	x
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

#[derive(Serialize, Deserialize, Debug)]
struct Segment{
	bottom: u64,
	top: u64,
	size: u64,
}