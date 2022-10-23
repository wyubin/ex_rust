fn main() {
	let number = 9;
	println!("Tell me about {}", number);
	let res = match number {
		1 => {println!("一...");number},
		2 | 3 | 5 | 7 | 11 => {println!("This is a prime and double it!");number*2},
		13...19 => {println!("A teen and triple it");number*3},
		_ => {println!("{} not in range",number);0},
	};
	println!("Final number: {}", res);
}
