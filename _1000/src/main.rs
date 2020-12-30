use std::io;
use std::io::BufReader;
use std::io::BufRead;

fn main() {

	let mut line = String::new();
	let mut stdin = BufReader::new(io::stdin());
  stdin.read_line(&mut line).unwrap(); 

  line.pop();	
  println!("{:?}", line.trim().split(" ").collect::<Vec<&str>>().iter().fold(0, |acc, x| acc + x.to_string().parse::<i64>().unwrap()));

}
