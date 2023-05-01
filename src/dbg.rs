#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn dbg_run() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    if !!(true) {
        println!("woshishabi");
    }

    dbg!(&rect1);
}
