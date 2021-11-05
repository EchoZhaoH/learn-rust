use std::{env};

mod config;

fn main() {
  let env: Vec<String> = env::args().collect();

  let args = config::Args::new(&env);

  println!("env args: {}", args.query);
}