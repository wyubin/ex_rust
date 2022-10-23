// 一樣的呼叫方式，只是要注意因為mroot 會被呼叫，所以要注意所有的 mod 是不是 pub
pub mod mod1;
mod mod2;
pub fn fn_pub(){
	println!("mod fn_pub");
}
#[allow(dead_code)]
fn fn_pri(){
	println!("mod fn_pri");
}
pub fn fn_mod2_pub(){
	println!("mod fn_mod2_pub");
	mod2::fn_pub();
}