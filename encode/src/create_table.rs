pub mod table {
  use std::{fs::File, io::Read};
	
	//TODO
	pub fn construct_table(data: &Vec<u8>) -> String{

		let huffman_tree = get_huffman_tree();
	
	}


	pub fn read_file_to_byte_vec(path: &str) -> std::io::Result<Vec<u8>>{

		let mut file = File::open(path)?;

		let mut buffer: Vec<u8> = Vec::new();

		file.read_to_end(&mut buffer)?;

		Ok(buffer)

	}

}