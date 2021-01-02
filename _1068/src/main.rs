use std::io;
use std::io::BufReader;
use std::io::BufRead;

fn main() {

	let mut line = String::new();
	let mut stdin = BufReader::new(io::stdin());
  stdin.read_line(&mut line).unwrap();


  let n = line.trim().parse::<i64>().unwrap();

  if n <=0{
  	println!("{:?}", (n..=1).collect::<Vec<i64>>().iter().fold(0,|x,acc| acc + x) );
  }
  else{
  	println!("{:?}", (1..=n).collect::<Vec<i64>>().iter().fold(0,|x,acc| acc + x) );

  }

} 