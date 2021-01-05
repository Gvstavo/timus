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

fn factorization(n: i64) -> Vec<i64>{

	let mut aux : i64 = n;

	let mut ret : Vec<i64> = Vec::new();

	if n% 2 ==0{
		ret.push(2);
		aux = n.checked_div(2).unwrap();
	}

	for i in (3..aux).step_by(2){

		if is_prime(i) && aux % i ==  0{
			ret.push(i);
		}

	}

	ret

} 

fn main() {

	println!("{:?}",factorization(510510) );

}
