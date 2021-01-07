use std::io;
use std::io::BufReader;
use std::io::BufRead;
fn minimal_product_digit(n: i64) -> i64{
	if n == 0 {return 10};
	if n < 10 { return n};
	
	let f : Vec<i64> = vec![2,3,5,7];
	let mut aux = n;
	let mut ret : Vec<i64> = Vec::new();
	for i in f{

		if  aux % i== 0 {

			loop{
				aux/=i;
				 ret.push(i);
				if aux % i != 0 {
					break;
				}
			}
		}
	};

	if ret.is_empty() || aux > 1 {
		return -1;
	}
	
	match ret.iter().fold(0,|acc,elem| if *elem == 3{acc + 1} else {acc}) {

		1 => {

			let index = ret .iter().position(|&x| x == 3).unwrap();

			if index > 0{

				ret.remove(index);

				let k = ret.get_mut(index - 1).unwrap();

				*k = 6;

				ret.sort();
				
			}
		}

		_ => ()

	}
	let mut v : Vec<i64> = Vec::new();	
	for i in ret{

		match i {

			2 | 3=> 

				match v.last_mut(){

					None => v.push(i),
					Some(last) if i ==3 && *last == 2 => v.push(i),
					Some(last) => {
						let valor = *last;
						if (valor * i) < 10{ 
							*last = valor*i
						}
						else{
							v.push(i);
						}
					}
				}
			_ => v.push(i)
		}
	}

		v.sort();
		v.iter().fold(0, |acc, elem| acc * 10 + elem)

}

fn main() {

	let mut line = String::new();
	let mut stdin = BufReader::new(io::stdin());

  stdin.read_line(&mut line).unwrap(); 

	let lim = line.trim().parse::<i64>().unwrap();

	println!("{:?}",minimal_product_digit(lim));

}
