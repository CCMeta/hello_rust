use std::env::args;
use std::process::{Command};

fn main() {
    println!("Usage:Please enter the command tail with this bin\n");
    let args_arr = args().skip(1);
    let output = Command::new("sh").arg("-c")
        .args(args_arr)
        .output()
        .expect("Fuck");
    let str_output = String::from_utf8(output.stdout)
        .expect("Fuck");
    println!("{str_output}");
    // for arg in &args_arr {
    //     println!("{arg}");
    // }
}