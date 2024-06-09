use std::{collections::HashMap, fs::File, io::Write};
use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    let vec = open_wav_file("table.aet");

    write_to_file(create_table(&vec));
}

fn open_wav_file(path: &str) -> Vec<i16>{

	let mut file = hound::WavReader::open(path).expect("hound cant open file");

	let x: Vec<i16> = file.samples::<i16>().map(|x| x.unwrap()).collect();

	x
}

fn create_table(vec: &Vec<i16>) -> HashMap<u16, Segment>{

    let map: HashMap<u16, Segment> = HashMap::new();

    todo!("create a lut for upper-/lower-bound and ");

    map
}

fn write_to_file(map: HashMap<u16, Segment>){
    let serialized = serde_json::to_string(&map).expect("cant convert json to HashMap");

    let mut file = File::create("table.aet").expect("cant open file table.aet");

    write!(file, "{}", serialized).expect("cant write to file");

}

#[derive(Serialize, Deserialize, Debug)]
struct Segment{
	bottom: u64,
	top: u64,
	size: u64,
}