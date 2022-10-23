// 建立一個物件
struct Pair(i32,i32);
// 一個比對 pair 是否在Pair物件中 的 trait
trait Contains {
	type A;
	type B;
	fn contains(&self,&Self::A,&Self::B) -> bool;
	// 因為在 differ fn 中會相減，所以需要定為 i32
	fn first(&self) -> i32;
	fn last(&self) -> i32;
}
impl Contains for Pair {
	type A=i32;
	type B=i32;
	fn contains(&self,a:&Self::A,b:&Self::B) -> bool {
		(&self.0 == a) && (&self.1 == b)
	}
	fn first(&self) -> i32 {self.0}
	fn last(&self) -> i32 {self.1}
}
fn differ<T:Contains>(pair: &T) -> i32 {
		pair.last()-pair.first()
}
fn main() {
	let n1 = 3;
	let n2 = 10;
	let pair1 = Pair(3,10);
	println!("Dose pair1 contain {} and {}: {}", &n1,&n2,pair1.contains(&n1,&n2));
	println!("first is: {}", pair1.first());
	println!("last is: {}", pair1.last());
	println!("differences is: {}", differ(&pair1));
}
