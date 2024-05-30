fn main() {
 
	let sample_vec: Vec<i16> = open_wav_file("testing/sample.wav");

  let mut acc_vec_pos: Vec<u32> = vec![0; 32768];
	let mut acc_vec_neg: Vec<u32> = vec![0; 32768];


	for s in sample_vec{
		match	(s as u16) >> 15 {
			0 => acc_vec_pos[ (s & 0x7FFF) as usize] +=1,
			1 => acc_vec_neg[ (s & 0x7FFF) as usize] +=1,
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

	//print entropy of input
	println!("Original File Size: {} Bytes\nEntropy of File: {}\nSum of Bytes: {}\nApproximated Size: {}", acc_vec.len()*2, entropy, sum, entropy/8f64 * sum);

}

fn open_wav_file(path: &str) -> Vec<i16>{

	//dbg!(hound::WavReader::open(path).unwrap().spec()); //DEBUG

	let x: Vec<i16> = hound::WavReader::open(path).unwrap().samples().map(|x| x.unwrap()).collect();
	x
}