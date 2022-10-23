// 建一個可以使用 Iterator 的物件
struct Fibo {
	n: u32,
	n1: u32,
}
impl Iterator for Fibo {
	type Item = u32;
	// 每次被 call next 除了修改物件數值以外，還會 return item，用 Option 跟Some包裝
	fn next(&mut self) -> Option<u32> {
		let new_n1 = self.n + self.n1;
		self.n = self.n1;
		self.n1 = new_n1;
		Some(self.n)
	}
}
// 產生 Fibo 起始點
fn fibo() -> Fibo {
	Fibo {n:1,n1:1}
}
fn main() {
	// 一般 slice 物件就有 Iterator
	let mut seqs = 0..3; // 必須為 mut 物件
	println!("1 next of seqs: {:?}", seqs.next());
	println!("2 next of seqs: {:?}", seqs.next());
	// skip 是 remove 前面 n 個 next，take(n)...output n item 後結束。
	for i in fibo().skip(3).take(3) {
		println!("get val by fibo: {}", i);
	}
	let array1 = [1,2,3,4];
	// 一般序列物件則可以用 iter() 去產生一個 Iterator 物件，再從此物件串接 trait
	for i in array1.iter() {
		println!("get val from array1 iter: {}", i);
	}
}
