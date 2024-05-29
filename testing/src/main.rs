fn main() {
 
	let sample_vec: Vec<i16> = open_wav_file("sample.wav");

	//dbg!(&sample_vec);

  let mut acc_vec: Vec<u32> = vec![0; 65536];

	for s in sample_vec{
		acc_vec[(s as i32 + i16::MAX as i32) as usize] +=1;
	}

	let prob_vec: Vec<f64> = acc_vec.iter()
		.map(|x| (*x as f64) / (acc_vec.len() as f64) )
		.collect();

	eprintln!("Sum of P: {}", prob_vec.iter().fold(0f64, |r, x| r+*x) );

}

fn open_wav_file(path: &str) -> Vec<i16>{
	let x: Vec<i16> = hound::WavReader::open(path).unwrap().samples().map(|x| x.unwrap()).collect();
	x
}