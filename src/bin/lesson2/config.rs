pub struct Args {
  pub query: String,
  pub filename: String,
}

impl Args {
  pub fn new(args: &[String]) -> Args {
    Args {
      query: args[1].clone(),
      filename: args[2].clone(),
    }
  }  
}
