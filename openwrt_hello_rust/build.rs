fn main() {
    println!("cargo:rustc-link-lib=hello"); //指定库
    // println!("cargo:rerun-if-changed=lib/hello.h");
    // need to copy libhello.so to /lib 
    let bindings = bindgen::Builder::default()
        .header("./lib/hello.h") //指定头文件，可以指定多个.h文件作为输入
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("./src/output.rs").unwrap(); //输出到那个目录
}
