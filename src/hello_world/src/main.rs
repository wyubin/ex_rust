fn main() {
	// print by order
	println!("one day have {} hours",25);
	// print by specific index
	println!("there are {0} apples in {1} boxes, all {1} boxes have {0} apples",25,3);
	// print by hash
	println!("hello {friend}, I'm {name}!",friend="Bill",name="wyubin");
	// assign format type
	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
	// print array variable
	println!("{} is the 0 index of {:?}", 1, [1,2]);
	// print by prefix
	println!("{number:>width$}", number=1, width=6);
	// print by struct
	#[allow(dead_code)]
	struct Structure(i32);
	//println!("This struct `{:?}` won't print...", Structure(3));
}
