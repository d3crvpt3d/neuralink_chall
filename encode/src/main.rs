use std::{fs::File, io::{BufWriter, Read, Write}};
use hashbrown::HashMap;
use serde::{Serialize, Deserialize};
use num_rational::Rational64;

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

//read lookup table
fn nums_pos_and_denom(path: &str) -> (HashMap<u16, Segment>, usize){

	let mut file: File = File::open(path).expect("cant read json file");
  let mut contents: String = String::new();
  file.read_to_string(&mut contents).expect("file is not json");

	let map: (HashMap<u16, Segment>, usize) = serde_json::from_str(&contents).expect("Failed to deserialize HashMap");

	map
}


//sequentially encodes byte Vec with arithmetic encoding
fn encode<W: Write>(data: Vec<i16>, stream: &mut BufWriter<W>, path: &str){

	let (segments, num_segments) = nums_pos_and_denom(path); //TODO: save bigrational instead of struct Segment

	stream.write_all(&create_arith_header(num_segments as u64)).expect("cant write header");

	let mut byte_buffer: [u8; 1] = [0]; //stores each not full byte. zB. '1','1','0' => [0,0,0,0,0,0,0,0] -> [1,1,0,0,0,0,0,0]

	let mut remaining_bytes_in_buf: u8 = 8;

	let mut o: Rational64 = Rational64::new(1, 1); //upper bound
	let mut s: Rational64 = Rational64::new(1, 1); //size
	let mut u: Rational64 = Rational64::new(0, 1); //lower bound

  let half = Rational64::new(1, 2);

	//NEEDS INTENSIVE TESTING
	data.iter().for_each(|&e| {

    let ee = e as u16;

		let curr_upper: Rational64 = Rational64::new(
      segments.get(&ee).unwrap().top as i64,
      num_segments as i64
    );

    let curr_lower = Rational64::new(
      segments.get(&ee).unwrap().bottom as i64,
      num_segments as i64
    );

    //calculate new position
		o = &u + &s * &curr_upper;
		u = &u + &s * &curr_lower;
		s = &s * (&o-&u);

		
    if o.numer() * 2 <= *o.denom(){//<= 0.5
      o *= 2;
      u *= 2;

			remaining_bytes_in_buf -= 1;//put 0
    }

    if u.numer() * 2 >= *o.denom() {
      o = (o - half) * 2;// (x - 1/2) * 2
      u = (u - half) * 2;

      byte_buffer[0] += 1 << (remaining_bytes_in_buf-1);////put 1
      remaining_bytes_in_buf -= 1;
    }


		//flush buffer
		if remaining_bytes_in_buf == 0{
			stream.write_all(&byte_buffer).expect("cant write a full byte to output");
			stream.flush().unwrap();
			byte_buffer[0] = 0;
			remaining_bytes_in_buf = 8;
		}

	});

	eprint!("|\n");
	stream.write_all(&byte_buffer).expect("cant write a full byte to output");
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