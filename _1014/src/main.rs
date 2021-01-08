use std::io;
use std::io::BufReader;
use std::io::BufRead;
fn minimal_product_digit(n: i64) -> i64{
	if n == 0 {return 10};
	if n < 10 { return n};

	let mut f : Vec<i64> = (2..=9).collect();
	let mut aux = n;

	f.reverse();

	let mut ret : Vec<i64> = Vec::new();	

	for i in f{

		if aux % i == 0 {
			loop{
				aux/=i;
				ret.push(i);
				if aux % i != 0 {break;}
			}
		}
	};

	if aux > 1 || ret.is_empty(){
		return -1
	};

	ret.reverse();
	ret.iter().fold(0, |acc, elem| acc * 10 + elem)


}

fn main() {

	let mut line = String::new();
	let mut stdin = BufReader::new(io::stdin());

  stdin.read_line(&mut line).unwrap(); 

	let lim = line.trim().parse::<i64>().unwrap();

	println!("{:?}",minimal_product_digit(lim));

}
