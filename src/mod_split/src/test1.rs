// 可以讀入所引用環境的 name space
use super::*;
// 需在 test fn 上面 tag #[test]
#[test]
fn my_test() {
	// 用 assert_eq 來 report 測試是否成功
	assert_eq!(1,mroot::mod1::fn_pub());
}