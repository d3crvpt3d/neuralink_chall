use std::{fs::File, io::{BufWriter, Read, Write}};
use hashbrown::HashMap;
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

	let mut o: u64 = segments.len() as u64; //upper bound
	let mut s: u64 = 1u64; //size
	let mut u: u64 = 0u64; //lower bound 

	let denom: u64 = o;

	//NEEDS INTENSIVE TESTING
	data.iter().for_each(|&e| {

		//if this throws an error it should crash please
		o = u + segments.get(&(e as u16)).unwrap().top;
		u = u + segments.get(&(e as u16)).unwrap().bottom;
		s = s * segments.get(&(e as u16)).unwrap().size;

		dbg!(o, u, s, o & (s >> 1));

		//fix large numbers
		while (!(o ^ u)) & (s >> 1) == denom {
			write!(stream, "{}", o & (s >> 1)).expect("cant write to writer");
	
			o = o >> 1;
			u = u >> 1;
			s = s >> 1;
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