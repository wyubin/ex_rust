// 實做一個 trait，基本上是定 spec
trait Animal {
	// 藉由 new 可以方便地預設數值，參數名字，字串通常用&'static str，return Self，也就是物件object
	fn new(name: &'static str) -> Self;
	// &self 是指已形成的 ref self
	fn name(&self) -> String;
	// 也可以寫好預設的功能
	fn intro(&self) {
		println!("My name is {}", self.name());
	}
}
// 也可以用 derive 來將物件標記預設的 trait
#[derive(Clone, Debug)]
struct Sheep{name: &'static str,naked:bool}
// 物件本身的 method
impl Sheep{
	fn is_naked(&self) -> bool {
		self.naked
	}
	fn shear(&mut self){
		if self.is_naked() {
			println!("{} is already naked...", self.name);
		} else {
			println!("{} shear now!!", self.name);
			self.naked = true;
		}
	}
}
impl Animal for Sheep {
	fn new(name: &'static str) -> Sheep {
		Sheep {name: name,naked: false}
	}
	fn name(&self) -> String {
		if self.is_naked() {
			format!("Naked {}",self.name)
		}else{
			self.name.to_owned()
		}
	}
}
fn main() {
	// 可以直接套用 Animal 的 new 去產生 Sheep
	let mut dolly: Sheep = Animal::new("Dolly");
	dolly.intro();
	dolly.shear();
	dolly.intro();
	let dolly2 = dolly.clone();
	dolly2.intro();
	//dolly2.shear(); // 不是 mut 就無法 shear
	println!("dolly: {:?}",dolly );
}
