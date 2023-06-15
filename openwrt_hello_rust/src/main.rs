use std::env::args;
use std::ffi::c_int;
use std::process::Command;

mod output;

fn c_say_hello() -> c_int {
    let shit = unsafe {
        output::say_hello()
    };
    shit
}

fn main() {
    let shit = c_say_hello();
    println!("{0}", shit);

    // return;
    let args_arr = args().skip(1);
    let output = Command::new("sh")
        .arg("-c")
        .args(args_arr)
        .output()
        .expect("Fuck");
    let str_output = String::from_utf8(output.stdout).expect("Fuck");

    {
        println!("==================================================");
        println!("Usage:Please enter the command tail with this bin");
        println!("==================================================");
        println!("\n");
        println!("{str_output}");

        // for arg in &args_arr {
        //     println!("{arg}");
        // }
    }
}
