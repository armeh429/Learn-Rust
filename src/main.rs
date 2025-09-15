use std::io;


fn main() {
  calculate_bmi();

}           

fn calculate_bmi() {
 
   println!("enter your weight in kg: ");
    let mut weight_str = String::new();
    io::stdin().read_line(&mut weight_str).expect("failed to read line");
    let _weight: f32 = weight_str.trim().parse().expect("please type a number!");


    println!("enter your height in cm: ");
    let mut height_str = String::new();
    io::stdin().read_line(&mut height_str).expect("failed to read line");
    let _height: f32 = height_str.trim().parse().expect("please type a number!");

    let height_m = _height / 100.0;
    let bmi = _weight / (height_m * height_m);
    println!("your bmi is: {:.2}", bmi);
}
