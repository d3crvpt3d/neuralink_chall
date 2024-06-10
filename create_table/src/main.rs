use std::{collections::HashMap, fs::File, io::{BufWriter, Write}};
use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    let vec = open_wav_file(args.get(1).expect("./create_table <file.wav> [<output file.aet>]"));

    let mut out_path = "table.aet";

    if args.len() == 3{
        out_path = args.get(2).unwrap();
    }

    write_to_file(create_table(&vec), out_path);
}

fn open_wav_file(path: &str) -> Vec<i16>{

	let mut file = hound::WavReader::open(path).expect("hound cant open file");

	let x: Vec<i16> = file.samples::<i16>().map(|x| x.unwrap()).collect();

	x
}

fn create_table(vec: &Vec<i16>) -> HashMap<u16, Segment>{

    let mut map: HashMap<u16, Segment> = HashMap::new();

    let mut freq: HashMap<usize, u64> = HashMap::new();

    //get occurences of segments into freq vec
    vec.iter().for_each(|&e| {

        let segment: usize = e as usize;

        //make sure key exists
        if !freq.contains_key(&segment){
            freq.insert(segment, 0);
        }

        let x = freq.get_mut(&segment).unwrap();
        *x += 1;
    });

    let mut u: u64 = 0;
    let mut o: u64 = 0;

    let freq_vec: Vec<(usize, u64)> = freq.into_iter().collect();
    
    freq_vec.into_iter().for_each(|e|{
        
        o = o + e.1;

        map.insert(
            e.0 as u16,
            Segment::new(u, o, o-u)
        );

        u = o;

    });

    map
}

fn write_to_file(map: HashMap<u16, Segment>, path: &str){
    let serialized = serde_json::to_string(&map).expect("cant convert json to HashMap");

    let mut stream = BufWriter::new(File::create(path).expect("cant open file table.aet"));

    dbg!(&serialized);

    write!(stream, "{}", serialized).expect("cant write to file");

}

#[derive(Serialize, Deserialize, Debug)]
struct Segment{
	bottom: u64,
	top: u64,
	size: u64,
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