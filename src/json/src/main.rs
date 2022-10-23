extern crate serde_json;

use std::fs::File;
use std::error::Error;
use std::path::Path;
use serde_json::Value;
// 直接寫fn 來讀取一個路徑的檔案，並輸出為 Result
fn load_json<P: AsRef<Path>>(path: P) -> Result<Value,Box<Error>> {
	let file = File::open(path)?;
	// 將檔案讀進程式並轉為 Value(是serde 的預設物件轉換格式)
	let v:Value = serde_json::from_reader(file)?;
	Ok(v)
}

fn main() {
	let res = load_json("./static/conf.json").unwrap();
	// 當讀取沒問題時，就可以印出來看看
	println!("{:?}", res);
	// 一樣將 json 寫入檔案
	let file = File::create("./static/write.json").unwrap();
	serde_json::to_writer(file,&res["url"]).unwrap();
}
