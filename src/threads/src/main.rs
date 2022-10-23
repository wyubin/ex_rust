use std::thread;
fn main() {
	let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
	// 先做一個收集各 thread result 的 vec
	let mut children = vec![];
	// 以跳行或空白分開資料
	let chunks = data.split_whitespace();
	// 用 iter 來進行 thread
	for (i,data_seg) in chunks.enumerate() {
		println!("line {} is \"{}\"", i,data_seg);
		// 用 move 的方式把資料轉進去 thread
		children.push(thread::spawn(move || -> u32 {
			let res = data_seg
				.chars()
				.map(|x|x.to_digit(10).expect("need to digit"))
				.sum();
			println!("sum of line {}: {}", i,res);
			res	
		}))
	}
	// 只是塞工作到thread 中，回收 value 的過程便是 reduce，join 會wait 到thread return
	// 然後用 unwrap 收取 res
	let mut temp_sum = vec![];
	for child in children {
		let tsum = child.join().unwrap();
		temp_sum.push(tsum);
	}
	// 所以下面的會在跑完thread 後才執行。
	println!("push order: {:?}", temp_sum);
	println!("Final sum result: {}", temp_sum.iter().sum::<u32>());
}
