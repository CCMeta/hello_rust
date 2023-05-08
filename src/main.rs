// include!("dbg.rs");
include!("ref.rs");
use std::process::Command;

fn main() {

    // for UNIX only
    // let mut result = Command::new("sh")
    //     .arg("-c")
    //     .arg("ls && uname -a")
    //     .output()
    //     .expect("msg");

    // for windows only
    let result = Command::new("cmd")
        .args(["/C", "echo hello"])
        .output()
        .expect("msg");


    let result_utf8 = String::from_utf8(result.stdout).expect("msg");
    println!("{result_utf8}");

    let mut x = 1;
    let mut y = 2;
    let z = "heelo".to_string();
    ref_test(&mut x, &mut y, &z);
    x = 5;
    y = 6;
    ref_test(&mut x, &mut y, &z);

    let s1 = "fuck".to_string();
    let mut s2 = "shit".to_string();
    let mut _ret = strcat(s1, &mut s2);
    println!("s2={s2}");
    // println!("{ret}");
    // ret = "modify".to_string();
    // println!("{ret}");

    let cn = "nihao";
    let en = "greeting";
    let lans = [en, cn];
    for lan in lans.iter() {
        println!("{}", lan);
    }
    println!("{}", fuck(cn.to_string()))
}

fn fuck(text: String) -> String {
    text
}
