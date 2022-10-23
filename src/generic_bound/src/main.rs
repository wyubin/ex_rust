use std::fmt::Debug;
// 先建立 trait，有 fn area 並 return f64
trait HasArea {
	fn area(&self) -> f64;
}
trait Empty {}
// 建立兩個物件，其中只有一個有 debug
#[derive(Debug)]
struct Rect { length: f64, height: f64 }
#[allow(dead_code)]
struct Tri  { length: f64, height: f64 }

// 在 Rect 加上 HasArea
impl HasArea for Rect {
	fn area(&self) -> f64 {
		self.length * self.height
	}
}
// Tri 加上 Empty
impl Empty for Tri {}
// 建立需要 HasArea 的 generic struct
struct S<T: HasArea>(T);
// 建立需要 Debug 的 fn
fn print_debug<T:Debug>(t: &T) {
	println!("{:?}", t);
}
// 可以限制只有 Tri 可以 nothing
fn nothing<T:Empty>(_: &T) -> &'static str {"nothing"}

// multiple bound 可以用 + 做雙重限制
fn print_area<T:Debug + HasArea>(t: &T){
	println!("{:?} have area:{}",t,t.area());
}
// bound 也可以用 where 來寫
fn print_area2<T,U>(t:&T,u:&U) where 
	T:Debug + HasArea,
	U:Empty {
	println!("only {:?} have area:{} but Tri jsut {}",t,t.area(),nothing(u));
}

fn main() {
	let trect = Rect {length:2.0,height:1.5};
	let _tri = Tri {length:2.0,height:1.5};
	println!("area of trect: {}", trect.area());
	print_debug(&trect);
	print_area(&trect);
	print_area2(&trect,&_tri);
	// print_debug(&_tri);
	let _srect = S(trect);
	// let stri = S(_tri);
	println!("nothing in Tri: {}", nothing(&_tri));
}
