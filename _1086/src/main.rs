use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

static mut largest_nth : i64 = 2;

fn is_prime(n: i64) -> bool{

	if n == 2 { return true};
	if n % 2 == 0 {return false};

	let s = (n as f64).sqrt().ceil();

	let lim = s as i64;

	for i in (3..=lim).step_by(2){



		if n % i == 0 { return false};

	}
	true

}

fn nth_prime_with_cache(n: i64 , cache: &mut HashMap<i64,i64>) -> i64{

	if cache.is_empty(){
		cache.insert(1,2);
		cache.insert(2,3);

	}

	let mut count = unsafe {largest_nth};


	let mut aux = *(cache.get(&count).unwrap());

	if n <= count {
		return *(cache.get(&n).unwrap())
	}

	loop{

		aux += 2;

		if is_prime(aux){
			unsafe{largest_nth += 1};
			count +=1;
			cache.insert(count , aux);

			if count == n {return aux}

		}

	}

}

fn main() {


	let mut line = String::new();
	let mut stdin = BufReader::new(io::stdin());

	let mut cache : HashMap<i64,i64> = HashMap::new();
	let mut l : Vec<i64> = Vec::new();

  stdin.read_line(&mut line).unwrap(); 

	let lim = line.trim().parse::<i64>().unwrap();

  for _ in 0..lim{

  	line.clear();
   	stdin.read_line(&mut line).unwrap(); 

 		l.push(line.trim().parse::<i64>().unwrap());

  };

  l.iter().for_each(|x| println!("{:?}", nth_prime_with_cache(*x,&mut cache)));

}
