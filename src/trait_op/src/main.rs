use std::ops;
#[derive(Debug)]
struct Point{val:i32}
// 讓point 可以藉由 operator:Add 互加
impl ops::Add for Point {
	type Output = Self;
	fn add(self,_rhs: Self) -> Self {
		Self {val:self.val+_rhs.val}
	}
}
// 用 drop 可以觀察物件 moved 的狀況
impl Drop for Point{
	fn drop(&mut self) {
		println!("destory {:?}", self);
	}
}
fn main() {
	let p3 = Point{val:3};
	let p5 = Point{val:5};
	let p1 = Point{val:1};
	println!("p3+p5+p1={:?}",p3 + p5 + p1);
}
