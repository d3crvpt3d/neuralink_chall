use std::io::Read;

use byteorder::{ByteOrder, LittleEndian};

fn main() {
 
	let (sample_vec, specs, wav_header) = open_wav_file("testing/sample.wav");

  let mut acc_vec_pos: Vec<u32> = vec![0; 32768];
	let mut acc_vec_neg: Vec<u32> = vec![0; 32768];


	for s in sample_vec{
		match	(s as u16) >> 15 {
			0 => acc_vec_pos[ (s & 0x7FFF) as usize] += 1,
			1 => acc_vec_neg[ (s & 0x7FFF) as usize] += 1,
			_ => eprintln!("matching +- failed"),
		}
	}

	//create single lut
	acc_vec_neg.append(&mut acc_vec_pos);
	let acc_vec: Vec<u32> = acc_vec_neg;

	//sums all elements in accumulation vector
	let sum = acc_vec.iter().fold(0, |acc, e| acc + *e) as f64;

	//creates the probability vector of acc_vec
	let prob_vec: Vec<f64> = acc_vec.iter()
		.map(|x| (*x as f64) / sum ) // div by sum of elements
		.collect();

	//dbg!(&prob_vec);
	
	let entropy = prob_vec.into_iter()
		.reduce( |acc, e| 
			if e != 0.0 {
				return acc + e * -e.log2();
			}else{
				return acc;
			}
		 )
		.unwrap();

	dbg!(&wav_header);

	//print entropy of input
	println!(
		"Original File Size: {} Bytes\n
		Entropy of File: {}\n
		Sum of Segments (u16): {}\n
		Approximated Compressed Size: {} Bytes or {:.2}%",
		wav_header.size+8,
		entropy,
		sum,
		(entropy*sum) as u32 / 8,
		(entropy*sum / 8f64)/wav_header.size as f64 * 100f64 );

}



fn open_wav_file(path: &str) -> (Vec<i16>, hound::WavSpec, Header){

	let mut file = hound::WavReader::open(path).expect("hound cant open file");

	let x: Vec<i16> = file.samples().map(|x| x.unwrap()).collect();


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

#[derive(Debug)]
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