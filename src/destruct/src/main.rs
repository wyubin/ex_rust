#[allow(dead_code)]
enum Color {
	Red,
	Blue,
	Green,
	RGB(u32,u32,u32),
	CMYK(u32,u32,u32,u32),
}
fn age() -> u32 {
	15
}
fn main() {
	let pair = (0,-2);
	// tuple destruct
	match pair {
		// 在 destruct 的同時可以做 pattern
		(0,y) => println!("`x` is 0 and y is {}", y),
		(x,0) => println!("`x` is {} and y is 0", x),
		_ => println!("`x` or `y` is not 0"),
	}
	// enum destruct
	let tcolor = Color::CMYK(1,2,3,5);
	match tcolor {
		Color::RGB(a1,a2,a3) => println!("R:{},G:{},B:{}", a1,a2,a3),
		// 只要 k 是 4的 color
		Color::CMYK(a1,a2,a3,4) => println!("C:{},M:{},Y:{} and with K is 4", a1,a2,a3),
		_ => println!("other color"),
	}
	// pointer destruct, 主要就是 pointer 跟 value 本身的對換
	let tref = &5;
	match tref {
		&val => println!("Got val by pointer destruct: {}", val),
	}
	match *tref {
		val => println!("Got val by de-reference: {}", val),
	}
	let mut mval = 6;
	match mval {
		ref mut mref => {
			*mref += 10;
			println!("revise val by reference destruct: {}", mref);
		}
	}
	// struct destruct
	struct Foo { x: (u32, u32), y: u32 }
	let foo = Foo {x:(1,2),y:3};
	let Foo { x: (a, b), y } = foo;
	println!("a = {}, b = {},  y = {} ", a, b, y);
	let Foo { y, .. } = foo;
	println!("y = {}", y);
	// data binding
	match age() {
		0 => println!("just born"),
		n@1...12 => println!("child({})", n),
		n@13...19 => println!("teenage({})", n),
		n => println!("older person({})", n),
	}
}
