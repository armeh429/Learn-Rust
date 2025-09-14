use std::io;


fn main() {
    println!("enter your weight in kg: ");
    let mut weight_str = String::new();
    io::stdin().read_line(&mut weight_str).expect("failed to read line");
    let _weight: f32 = weight_str.trim().parse().expect("please type a number!");
}

