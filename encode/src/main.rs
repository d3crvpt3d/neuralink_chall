use std::{fs::File, io::{BufReader, Read, Write}};
use num_rational::Rational32;
use hashbrown::HashMap;
use serde::{Serialize, Deserialize};

//ALL IS LITTLE ENDIAN
fn main(){

	let args: Vec<String> = get_args();

	let sample_vec = open_wav_file(args.get(1).unwrap());

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

//read lookup table (I LOVE SERDE AND HASHBROWN)
fn nums_pos_and_denom(path: &str) -> HashMap<u16, Segment>{

	let mut file = File::open(path).expect("cant read json file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("file is not json");

	let map: HashMap<u16, Segment> = serde_json::from_str(&contents).expect("Failed to deserialize HashMap");

	map
}


//sequentially encodes byte Vec with arithmetic encoding
fn encode(data: Vec<i16>) -> Vec<u8>{
	
	let mut writer = Vec::new();

	let segments: HashMap<u16, Segment> = nums_pos_and_denom("table.aet");

	let mut o: u64 = segments.len() as u64; //upper bound
	let mut s: u64 = 1u64;
	let mut u: u64 = 0u64;

	let denom: u64 = o; //lower bound //size

	//NEEDS INTENSIVE TESTING
	data.iter().for_each(|&e| {

		o = u + segments.get(&(e as u16)).unwrap().top;
		u = u + segments.get(&(e as u16)).unwrap().bottom;
		s = s * segments.get(&(e as u16)).unwrap().size;

		//fix large numbers
		while (!(o ^ u)) & (s >> 1) == denom {
			write!(&mut writer, "{}", o & (s >> 1)).expect("cant write to writer");
	
			o = o >> 1;
			u = u >> 1;
			s = s << 1;
		}
	});

	[create_arith_header(segments.len() as u64), writer].concat()

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

impl Segment {
		fn new(bottom: u64, top: u64, size: u64) -> Self{
			Segment{
				bottom,
				top,
				size,
			}
		}
}

#[derive(Serialize, Deserialize, Debug)]
struct Segment{
	bottom: u64,
	top: u64,
	size: u64,
}