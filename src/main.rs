// include!("dbg.rs");
include!("ref.rs");
use std::process::Command;
use futures::executor::block_on;

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
    let future = fuck(cn.to_string());
    println!("{}", future);
}

fn fuck(text: String) -> String {
    let mut x = String::from("junmajinlong");
    let x_mut1 = &x;
    println!("{}", block_on(name(x_mut1)));
    println!("{}", x_mut1);
    println!("{}", x_mut1);
    println!("{}", x_mut1);
    println!("{}", x_mut1);
    println!("{}", x_mut1);
    let x_mut2 = &mut x;
    let x_mut3 = &mut x;
    println!("{}", x_mut3);

    text
}

async fn name(x: &String) -> String {
    x.to_string()
}
