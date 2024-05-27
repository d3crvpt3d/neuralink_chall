use rustfft::{num_complex::Complex, FftPlanner};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use bincode::serialize;
use huffman_compression::BitWriter;
use huffman_compression::codecs::DefaultEncoder;
use huffman_compression::tree::TreeBuilder;
use huffman_compression::codecs::Encoder;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3{
        eprintln!("Usage: <input file> <output file> (args.len: {})", args.len());
        return;
    }

    let samples: Vec<i16> = hound::WavReader::open(&args[1]).unwrap().samples::<i16>().map(|x| x.unwrap()).collect();

    let mut buffer = samples;

    let mut planner = FftPlanner::new();

    let fft = planner.plan_fft_forward(buffer.len());

    fft.process(&mut buffer);

    compress_and_write_huffman(buffer, Path::new(&args[2])).unwrap();

}

fn compress_and_write_huffman(data: Vec<i16>, output_path: &Path) -> Result<(), Box<dyn Error>> {
    // Serialize the data to bytes
    let serialized_data = serialize(&data)?;

    // Build Huffman tree
    let mut tree_builder = TreeBuilder::new();
    tree_builder.extend_from_slice(&serialized_data);
    let tree = tree_builder.build_tree();

    // Encode the data
    let mut encoder = DefaultEncoder::from_tree(tree);
    let mut compressed_data = Vec::new();
    {
        let mut bit_writer = BitWriter::new(&mut compressed_data);
        encoder.encode(&mut bit_writer, &serialized_data)?;
        bit_writer.finish()?;
    }

    // Write compressed data to file
    let mut file = File::create(output_path)?;
    file.write_all(&compressed_data)?;

    Ok(())
}