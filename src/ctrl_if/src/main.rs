fn main() {
	let n = 50;
	let big_n = if n < 10 && n > -10 {
		println!("n={} is a x-small number, increase 100-fold", n);
		100*n
	} else if n < 100 && n > -100 {
		println!("n={} is a small number, increase 10-fold", n);
		10*n
	} else {
		println!("n={} is a normal number, half the number", n);
		n/2
	};
	println!("{} -> {}", n, big_n);
}
