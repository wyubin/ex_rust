use std::fmt::Debug;
// 是一個泛用type 的 struct
struct Simbox<T>{
	val:T
}
// 具泛用 type 的 func
fn g_print<T>(_input:Simbox<T>){}
// impl
impl <T> Simbox<T> {
	fn value(&self) -> &T {&self.val}
}
// trait
trait Ds<T> {
	fn do_some(self,_:T);
}
// for Any type
impl <T,U> Ds<T> for U {
	fn do_some(self,_:T){
		println!("do something for Simbox");
	}
}
fn main() {
	let tint = Simbox{val:6};
	let tchar = Simbox{val:'A'};
	let tf = Simbox{val:6.123};
	let tstr = Simbox{val:"AA"};
	g_print(tchar);
	// 也可以指定 type
	g_print::<i32>(tint);
	// 使用 method
	println!("Simbox value: {}", tf.value());
	tf.do_some(tstr);
}
