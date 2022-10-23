/*
以 mod 呼叫在相同資料夾的其它 rs, mroot 的話，會去搜尋 mroot.rs 或是 mroot/mod.rs，則會形成所呼叫名字的變數
*/
mod mroot;
// 要加入 test 
#[cfg(test)]
mod test1;
#[cfg(test)]
mod test2;

fn main() {
	mroot::fn_pub();
	mroot::mod1::fn_pub();
	mroot::fn_mod2_pub();
	//mroot::mod2::fn_pub;
}
