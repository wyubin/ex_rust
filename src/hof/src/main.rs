fn main() {
	println!("Find the sum of all the squared odd numbers under 1000");
	let tmax = 1000;
	// by loop
	let mut acc = 0;
	for n in 0..{
		let tsq = n*n;
		if tsq >=tmax {
			break;
		}else if n % 2 == 1 {
			acc += tsq;
		}
	}
	println!("count by loop: {}", acc);
	// by hof
	let res:i32 = (0..).map(|n|n*n)
		.take_while(|&n|n<tmax)
		.filter(|&n|n % 2 == 1)
		.fold(0,|acc,n|acc+n);
	println!("count by hof: {}", res);
}
