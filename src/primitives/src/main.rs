// A tuple struct
struct Pair(i32, f32);
// A struct with two fields
#[derive(Debug)]
struct Point {
	x: f32,
	y: f32,
}
#[derive(Debug)]
struct Rectangle {
	p1: Point,
	p2: Point,
}
fn main() {
	let point = Point { x: 0.3, y: 0.4 };
	println!("point coordinates: ({}, {})", point.x, point.y);
	let new_point = Point { x: 0.1, ..point };
	// `new_point.y` will be the same as `point.y` because we used that field from `point`
	println!("second point: ({}, {})", new_point.x, new_point.y);
	let Point { x: my_x, y: my_y } = point;
	println!("explore by point: ({}, {})", my_x, my_y);
	let trectangle = Rectangle {
		// struct instantiation is an expression too
		p1: Point { x: my_y, y: my_x },
		p2: new_point,
	};
	println!("print a rectangle: {:?}", trectangle);
	let pair = Pair(1, 0.1);
	let Pair(integer, decimal) = pair;
	println!("explore by Pair(1, 0.1): ({}, {})", integer, decimal);
}
