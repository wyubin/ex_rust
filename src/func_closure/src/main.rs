fn main() {
	fn tfunc(i:i32) -> i32 {i+1}
	let tint = 3;
	// 能夠直接傳入外面變數
	let tclosu = |i| i+tint+1;
	let tone = || tint;
	println!("call by function: {}", tfunc(1));
	println!("call by closures: {}", tclosu(1));
	println!("always tint: {}", tone());
	// 預設傳 &T，但需要時就會傳 T，對 non-copy object 就會 moved而影響到之後物件的使用
	use std::mem;
	let cprint = || {
		println!("use tint: {}", tint);
		mem::drop(tint);
	};
	// copy tint by 傳T
	cprint();
	cprint();
	let movable = Box::new(3);
	let mprint = || {
		println!("use movable: {}", movable);
		mem::drop(movable);
	};
	// moved by 傳T
	mprint();
	//mprint();
	// 另外可以用 move 將傳入變數轉換 ownship，參數可以多次使用，但外面就不能用了
	let haystack = vec![1, 2, 3];
	let contains = move |needle| haystack.contains(needle);
	println!("{}", contains(&1));
	//println!("{:?}", haystack);
}
