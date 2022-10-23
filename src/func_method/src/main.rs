#[derive(Debug)]
struct Point{
	x:i32,
	y:i32,
}
impl Point {
	fn origin() -> Point {
		Point{x:0,y:0}
	}
	// 用 new 就可以簡單地建立 point，而不需要指定 x,y，是常用的封裝技巧
	fn new(x: i32, y: i32) -> Point {
		Point{x:x,y:y}
	}
}
#[derive(Debug)]
struct Rect(Point,Point);
impl Rect {
	fn area(&self) -> i32 {
		let Point{x:x1,y:y1} = self.0;
		let Point{x:x2,y:y2} = self.1;
		((x1 - x2) * (y1 - y2)).abs()
	}
	fn peri(&self) -> i32 {
		let Point{x:x1,y:y1} = self.0;
		let Point{x:x2,y:y2} = self.1;
		2 * ((x1 - x2).abs() + (y1 - y2).abs())
	}
	fn translate(&mut self, x: i32, y: i32) {
		self.0.x += x;
		self.1.x += x;

		self.0.y += y;
		self.1.y += y;
	}
}
#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
	fn destroy(self) {
		// consume these two box
		let Pair(p1,p2) = self;
		println!("Destroy Pair({},{})", p1,p2);
	}
}

fn main() {
	let rect1 = Rect(Point::origin(),Point::new(3,4));
	println!("rect1 perimeter: {}", rect1.peri());
	println!("rect1 area: {}", rect1.area());
	let mut rect2 = Rect(Point::origin(),Point::new(1,1));
	rect2.translate(2, 3);
	println!("rect2: {:?}", rect2);
	let p1 = Box::new(1);
	let p2 = Box::new(2);
	let pair = Pair(p1,p2);
	pair.destroy();
	//println!("{:?}", pair);
	// pair have moved by destroy call 'self' insteal call &self
}
