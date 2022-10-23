// 在一般的錯誤處理中，常會遇到的是 None 物件，通常是用 Some(None) 封裝
// 我們可以用 Option 配合 match 來做錯誤處理
fn num_test0(num: Option<&str>) {
	match num {
		Some("0") => println!("Zero!!!"),
		None => println!("No number input"),
		Some(c) => println!("input: {}", c),
	}
}
// 如果只是要分辯 None 與否(valid input)，其實可以用 Option 的.map
// 如果是多層處理，每層僅需要辨認不為None，可以用.map，如果不為 None，
// 會進行所 input 的 fn，否則 return None
fn num_test1(num: Option<&str>){
	num.map(|_| "Number").map(|x|println!("I'm a {}", x));
}
fn main() {
	let counts = vec![Some("100"),Some("0"),None];
	for i in counts {
		num_test0(i);
		num_test1(i);
	}
}
