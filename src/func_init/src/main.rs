fn main() {
	divise_list(20);
}

// check divise
fn is_divise(a:u32,b:u32) -> bool {
	if b == 0 {
		return false;
	}
	a % b == 0
}

// divise_list, return () by default
fn divise_list(n: u32){
	for i in 1..n+1 {
		if is_divise(i,5){
			println!("{} is divise by 5", i);
		}
	}
}