use std::error;
use std::fmt;
// 自訂錯誤類型
#[derive(Debug, Clone)]
struct FirstE;
// 加上 Error 及 display trait
impl error::Error for FirstE {
	fn description(&self) -> &str {
		"No first item"
	}
	// return None if error
	fn cause(&self) -> Option<&error::Error> {
		None
	}
}
impl fmt::Display for FirstE {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"invalid first item for double")
	}
}
// 針對 FirstE 的 Result
type ResFirst<T> = Result<T, FirstE>;
// test1，把其中的 error 全部轉成 FirstE，這樣才能符合同一種 return type
// 但第二個錯誤是 parse int 錯誤
fn test1(vec: Vec<&str>) -> ResFirst<i32> {
	vec.first().ok_or(FirstE) //if error 轉成 FirstE
		.and_then(|x|x.parse::<i32>()
			.map_err(|_|FirstE)
			.map(|x| 2*x)
		)
}
// test2 把 Result中封裝 Box，就可以統一 error type
type ResBox<T> = Result<T, Box<error::Error>>;
fn test2(vec: Vec<&str>) -> ResBox<i32> {
	vec.first().ok_or_else(|| FirstE.into()) // ok_or_else 是 return fn if None, .into() 可以轉成 Box
		.and_then(|x|x.parse::<i32>()
			.map_err(|e|e.into()) // 一樣把 parseint error 轉成 box，就可以正確顯示錯誤了
			.map(|x| 2*x)
		)
}
// ? 可以直接把return Err(e) 轉成 Box，就不用 into 了，也可以提早 return
fn test3(vec: Vec<&str>) -> ResBox<i32> {
	let step1 = vec.first().ok_or(FirstE)?;
	let step2 = step1.parse::<i32>()?;
	Ok(2*step2)
}
fn print_res(result: ResFirst<i32>){
	match result {
		Ok(num) => println!("double num: {}", num),
		Err(e) => println!("Error: {}", e),
	}
}
fn print_res2(result: ResBox<i32>){
	match result {
		Ok(num) => println!("double num: {}", num),
		Err(e) => println!("Error: {}", e),
	}
}

fn main() {
	let nums = vec![vec!["10","20"],vec!["","30"],vec![]];
	for i in nums {
		print_res(test1(i));
	}
	let nums2 = vec![vec!["10","20"],vec!["","30"],vec![]];
	for i in nums2 {
		print_res2(test2(i));
	}
	let nums3 = vec![vec!["10","20"],vec!["","30"],vec![]];
	for i in nums3 {
		print_res2(test3(i));
	}
	// 最後，在 vec 物件中，我們可以用內建的 map 方式做 valid
	let strings = vec!["tofu", "93", "18"];
	let numbers: Vec<_> = strings
		.into_iter()
		.map(|x|x.parse::<i32>())
		.filter_map(Result::ok) // 可以直接把 ok 的抽出並unwrap
		.collect();
	println!("Results: {:?}", numbers);
}

