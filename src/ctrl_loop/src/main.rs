fn main() {
	let mut count = 0;
	println!("start to loop...");
	let res = 'out_loop: loop{
		count += 1;
		if (count % 3) == 0 {
			println!("{} 是3的倍數",count);
			'inner_loop: loop {
				println!("enter inner loop!");
				if count >6 {
					println!("exit outer loop and get double!");
					break 'out_loop count*2;
				}
				println!("exit inner loop");
				break;
			}
			continue;
		}
		println!("count: {}", count);
	};
    println!("result: {}",res);
}
