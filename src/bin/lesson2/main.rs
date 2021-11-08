use std::{env, process};

mod config;

fn main() {
  let env: Vec<String> = env::args().collect();

  let args = config::Args::new(&env).unwrap_or_else(|error| {
    println!("Problem parsing arguments: {}", error);
    process::exit(1);
  });

  println!("env args: {}", args.query);

  if let Err(e) = config::run(args) {
    println!("Application error: {}", e);

    process::exit(1);
  }
}