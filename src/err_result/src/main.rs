use std::num::ParseIntError;
type ResParse<T> = Result<T, ParseIntError>;
// Result 是增強版的 Option，也可以用 .unwrap() 做 panic...
// 主要是 Ok<T> 或是 Err<E>，可用 match 做簡單的 handle
fn test_parse1(str1: &str, str2: &str) -> ResParse<i32>{
	match str1.parse::<i32>() {
		Ok(num1) => {
			match str2.parse::<i32>() {
				Ok(num2) => {
					Ok(num1*num2)
				},
				Err(e) => Err(e),
			}
		},
		Err(e) => Err(e),
	}
}
// 也可以用 .and_then(純粹 return input fn) 來做程式碼簡化, .map(將 fn return 包成 Result)
fn test_parse2(str1: &str, str2: &str) -> ResParse<i32>{
	str1.parse::<i32>().and_then(|num1|{
		str2.parse::<i32>().map(|num2|num1*num2)
	})
}
// test_parse2 中，都需要到 str2parse 結束後才會 return，
// 可以使用 ? 讓 Result local return Err()，或是直接 return 計算結果
fn test_parse3(str1: &str, str2: &str) -> ResParse<i32> {
	let num1 = str1.parse::<i32>()?;
	let num2 = str2.parse::<i32>()?;
	Ok(num1*num2)
}
fn print_res(result: ResParse<i32>){
	match result {
		Ok(num) => println!("multiple num: {}", num),
		Err(e) => println!("Error: {}", e),
	}
}
fn main() {
	let nums = vec![["10","20"],["30",""]];
	for i in nums {
		print_res(test_parse1(i[0],i[1]));
		print_res(test_parse2(i[0],i[1]));
		print_res(test_parse3(i[0],i[1]));
	}
}
