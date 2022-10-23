enum Foo {
	Baz,
	Bar(i32),
}

fn main() {
	let mut number = Some(2);
	let letter: Option<i32> = None;
	if let Some(i) = number {
		println!("number have Option: {}", i);
	}
	if let Some(i) = letter {
		println!("letter have Option: {}", i);
	}else{
		println!("letter is None");
	}
	// 也可以用在 enum
	let f1 = Foo::Bar(7);
	if let Foo::Bar(i) = f1 {
		println!("value is {}", i);
	}
	while let Some(i) = number {
		if i > 9 {
			println!("more than 9,quit");
			number = None;
		} else {
			println!("from {} to 9",i);
			number = Some(i + 1);
		}
	}
}
