fn ref_test(x: &mut i32, y: &mut i32, z: &String) -> i32 {
    *x = *x + 1;
    *y = *y + 1;

    println!("x={x} y={y} z={z}");

    0
}

fn strcat(_s1: String, s2: &mut String){
  let mut s3 = s2;
  println!("s3_1={s3}");
  let mut binding = "s3".to_string();
  s3 = &mut binding;
  println!("s3_2={s3}");
}
