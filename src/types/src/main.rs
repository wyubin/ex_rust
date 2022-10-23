#![allow(overflowing_literals)]
#[allow(non_camel_case_types)]

fn main() {
	let tf32 = 65.4321_f32;
	// 用 as 將 f32 轉成 u8 給 tint
	let tint = tf32 as u8;
	// f32 用 as 並不能直接轉char，就只能從 u8 轉
	let tchar = tint as char;
	println!("Casting: {} -> {} -> {}", tf32, tint, tchar);
	// type 關鍵字可以將類型設定 alias
	type u8_t = u8;
	/* 數字間格式轉換時，如果轉換後的格式之最大限制超過後，則會將轉換前的格式
		累減或累加到範圍內再進行轉換
	*/
	// 預設整數為 u32，u16的範圍是 0-2^16(1024)
	println!("1000 as a u16 is: {}", 1000 as u16);
	// u8是 0-2^8(256), 1000-256-256-256=232
	println!("1000 as a u8 is: {}", 1000 as u8);
	println!("same with 1000 mod 256 is : {}", 1000 % 256);
	println!("-1 as a u8_t is: {}", -1i8 as u8_t);
	// 如果是剛好在最大值跟最小值間，會轉成最小值
	println!("128 as a i8 is: {}", 128 as i8);

	// inference 系統是可以從之後的操作來定義物件的型態
	let mut tvec = Vec::new();
	tvec.push(tchar);
	// push 之後就設定 tvec 為 Vec<char> 了
	tvec.push('B');
	println!("{:?}", tvec);
}
