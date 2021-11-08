use std::{fs, error::Error};

pub struct Args {
  pub query: String,
  pub filename: String,
}

impl Args {
  pub fn new(args: &[String]) -> Result<Args, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    Ok(Args {
      query: args[1].clone(),
      filename: args[2].clone(),
    })
  }  
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(args.filename)?;
  println!("With text: \n {}", contents);
  Ok(())
}

