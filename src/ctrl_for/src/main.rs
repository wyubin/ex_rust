fn main() {
	let tvec = 1..=3;
	let mut nvec = vec!["Bob", "Frank", "Ferris"];
	println!("start loop tvec(by into_iter)");
	for i in tvec {
		// 預設使用 into_iter()，tvec moved
		println!("tvec:{}", i);
	}
	println!("start loop nvec by iter()");
	for i in nvec.iter() {
		match i {
			&"Ferris" => println!("There is a rustacean among us!"),
			_ => println!("Hello {}", i),
		}
	}
	println!("start loop nvec by iter_mut()");
	// nvec can use again
	for i in nvec.iter_mut() {
		match i {
			&mut "Ferris" => {
				println!("There is a rustacean among us!");
				// 可以改名
				*i="son of Ferris";
			},
			_ => println!("Hello {}", i),
		}
	}
	println!("Ferris change to {}",nvec[2] );
}
