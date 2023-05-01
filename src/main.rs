// include!("dbg.rs");
use std::process::{Command};

fn main() {

    let result = Command::new("sh").arg("-c").arg("ls && uname -a").output().expect("msg");
    let result_utf8 = String::from_utf8(result.stdout).expect("msg");
    println!("{result_utf8}");

    let cn = "nihao";
    let en = "greeting";
    let lans = [en, cn];
    for lan in lans.iter() {
        println!("{}", lan);
    }
    println!("{}",fuck(cn.to_string()))
}

fn fuck(text:String) -> String {
    text
}
