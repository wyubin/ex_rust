fn apply_none<F>(tc: F) where F:FnOnce() {
	tc();
}
// 也可以這樣寫
fn apply_2none<F:FnOnce()>(tc: F) {
	tc();
}
fn tfunc(){
	println!("I'm Function!!");
}
fn apply_3<F>(tc: F) -> i32 where F:Fn(i32) -> i32 {
	tc(3)
}
// out a closure，需要 Box 封裝，通常需要 move 來轉移資料權限
fn get_fnmut(x:i32) -> Box<FnMut()> {
	Box::new(move || println!("x in closure: {}",x))
}
fn main() {
	// 可以用在一般 function
	apply_none(tfunc);
	apply_2none(tfunc);
	// 做一個需要傳T的closure
	let mut farewell = "goodbye".to_owned();
	use std::mem;
	let diary = || {
		println!("First:{}",farewell);
		farewell.push_str("!!!");
		println!("Then:{}",farewell);
		mem::drop(farewell);
	};
	apply_none(diary);
	println!("3 doubled: {}", apply_3(|x| 2*x));
	// out closure
	let mut x=11;
	let mut tclos=get_fnmut(x);
	x=32;
	println!("x in main: {}", x);
	// x已經 copy 所以不會被改。
	tclos();
}
