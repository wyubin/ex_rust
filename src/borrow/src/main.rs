// 下面是指允許物件 copy 的方式傳 T
//#[derive(Clone, Copy)]
#[derive(Debug)]
struct Point(i32);
fn borrow_point(x:&Point){
	println!("this is: {:?}", x);
}
fn move_point(x:Point){
	println!("Move the point:{:?}", x);
}
fn mutate_point(x:&mut Point){
	x.0 = 10i32;
}
fn main() {
	let pot1 = Point(5i32);
	borrow_point(&pot1);
	// 沒辦法改成 &mut, 因為本來的變數沒有 mut 權限
	// mutate_point(&mut pot1);
	{
		// 被 borrow 之後仍然可以被 ref
		let _ref_pot1 = &pot1;
		let _ref_pot2 = &pot1;
		// 被 ref 之後就沒辦法被 move 了
		// move_box_i32(box5);
	}
	// 一旦 ref 被 destory 後就可以被 move 了
	let mut pot2 = pot1;
	// 被 move 之後就沒辦法再move
	// move_point(pot1);
	{
		let _ref_pot2 = &pot2;
		// 當被 &T 時，原本的 variable 也不能 mutate 了
		// pot2.0 = 6i32;
		// 也不能被 &mut
		//let _ref2_pot2 = &mut pot2;
	}
	// 當 &T destory 了，就可以 mutate 了
	pot2.0 = 6i32;
	mutate_point(&mut pot2);
	move_point(pot2);
}
