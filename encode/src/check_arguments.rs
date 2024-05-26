pub mod arg_check {
  
  pub fn has_right_arguments(args: &Vec<String>) -> bool{

    if std::env::args().len() != 3{
      eprintln!("specify input and output path");
      return false;
    }

    if !std::fs::File::create_new(&args[1]).is_ok(){
      eprintln!("cant open {}", &args[1]);
      return false;     
    }

    if !std::fs::File::create_new(&args[2]).is_ok(){
      eprintln!("cant open {}", &args[1]);
      return false;
    }

    true

  }

}