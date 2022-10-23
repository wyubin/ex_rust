fn main() {
	let new_live;
    let mut long_live = 1;
	// 用簡單的 block 做 scope
	{
		let short_live = 2;
		println!("inner short: {}", short_live);
		// 用 let 讓 block 內也可以有同一個 variable name(shadowing)
		let long_live = 5.11_f32;
		println!("inner long: {}", long_live);
		// new_live 在 block 裡面也可以宣告型別，注意沒有 let
		new_live = short_live*2
	}
	println!("new live: {}", new_live);
	// new_live 不能被改
	// new_live += 2;
	// short_live 在外面就是不在 scope 內
	//println!("outer short: {}", shortLive);
	println!("outer long: {}", long_live);
	long_live+=1;
	println!("revised long: {}", long_live);
}
