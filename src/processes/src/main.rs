use std::io::prelude::*;
use std::process::{Command, Stdio,Child};
// 寫一個 pipe fn
fn pipe_in(tcmd:Child, in_str:&str) -> std::io::Result<String> {
	let _pw = tcmd.stdin.unwrap().write_all(in_str.as_bytes())?;
	let mut s = String::new();
	let _pr = tcmd.stdout.unwrap().read_to_string(&mut s)?;
	Ok(s)
}
fn main() {
	let tout = Command::new("rustc").args(&["--version"]).output()
		.unwrap_or_else(|x|{
			panic!("failed to exec:{}",x)
		});
	if tout.status.success(){
		let s = String::from_utf8_lossy(&tout.stdout);
		print!("rustc success and output was: {}",s);
	}else{
		let s = String::from_utf8_lossy(&tout.stderr);
		print!("rustc failed and output was: {}",s);
	}
	// 寫一個可以用來 pipe 的 process
	let tproc = Command::new("wc").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
	let res = pipe_in(tproc,"the quick brown fox jumped over the lazy dog\n");
	println!("pipe result: {}", res.unwrap());
}
