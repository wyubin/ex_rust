use std::convert::From;
use std::string::ToString;

#[derive(Debug)]
struct Number {
	value: i32,
}
// 可以藉由 std::convert::From 來建立轉換 function
impl From<i32> for Number {
	fn from(item: i32) -> Self {
		Number {value:item}
	}
}
// 另外也可以用 std::string::ToString 來轉成字串
impl ToString for Number {
	fn to_string(&self) -> String {
		format!("Number of value: {:?}", self.value)
	}
}


fn main() {
	// 字串之間是有功能可以互相轉換的
	println!("from &'static str to String : {}", String::from("hello"));
	// 也可以用 parse() 來轉換成其他 type，可以依照目標型態或是自行指定型態
	let tint: i32 = "10".parse().unwrap();
	let tint2 = "20".parse::<i32>().unwrap();
	// 使用 ::from 進行轉換
	let tnum = Number::from(tint+tint2);
	println!("tnum: {:?}",tnum);
	// 當建立 from ，就自動會在 from 的標的物件加上 .into()，來轉換成所設定的物件格式
	let ttnum: Number = 30.into();
	println!("ttnum by into: {:?}", ttnum);
	// 也可以用 to_string 來印
	println!("ttnum by to_string(): {}", ttnum.to_string());
}
