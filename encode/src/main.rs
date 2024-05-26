mod create_table;
mod check_arguments;
mod huffman;

use check_arguments::arg_check;
use create_table::table;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    if !arg_check::has_right_arguments(&args){
        return;
    }

    let lut = table::construct_table(
        &table::read_file_to_byte_vec(&args[1]).unwrap()
    );



}