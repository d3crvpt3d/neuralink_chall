use std::fs::File;

fn main() {

    //check is arguments are specified correctly and no file errors happen
    if std::env::args().len() != 3{
        eprintln!("specify input and output path");
        return;
    }

    let args: Vec<String> = std::env::args().collect();

    if std::fs::File::create_new(&args[1]).is_ok(){
        let in_file = File::create_new(&args[1]).unwrap();
    }else {
        eprintln!("cant open {}", &args[1]);
        return;       
    }

    if std::fs::File::create_new(&args[2]).is_ok(){
        let out_file = File::create_new(&args[2]).unwrap();
    }else {
        eprintln!("cant open {}", &args[1]);
        return;       
    }
    //check is arguments are specified correctly and no file errors happen

    

}