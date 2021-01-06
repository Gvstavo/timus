use std::io;
use std::io::BufReader;
use std::io::BufRead;
fn minimal_product_digit(n: i64) -> i64{

	let f : Vec<i64> = vec![2,3,5,7];

	let mut aux = n;
	let mut ret : Vec<i64> = Vec::new();
	let mut count = 0;

	for i in f{

		if  aux % i== 0 {

			loop{

				aux/=i;
				count +=1;
				match ret.last_mut(){

					None => ret.push(i),
	
					Some(last) if count > 1=>{
						let valor = *last;
						if (valor * valor) < 10{ 
							*last = valor*valor
						}
						else{
							ret.push(i);
						}
					}
					Some(_last) => ret.push(i)
				}
				if aux % i != 0 {
					count = 0;
					break;
				}
			}
		}
	}

	if ret.is_empty(){
		-1
	}
	else{
		ret.sort();
		ret.iter().fold(0, |acc, elem| acc * 10 + elem)
	}

}

fn main() {

	let mut line = String::new();
	let mut stdin = BufReader::new(io::stdin());

  stdin.read_line(&mut line).unwrap(); 

	let lim = line.trim().parse::<i64>().unwrap();

	println!("{:?}",minimal_product_digit(lim));

}
