// use std::io;
// use rand::Rng;


fn main() {
    //   calculate_bmi();

    // let _ = random_number();

    loop_exmple();


}           

fn loop_exmple() {
    let mut count = 0;
    loop {
        
        if count >= 100{
            break;
        }else {
           
            println!("{} - hello world" , count); 
             count += 1;
        }
        
    }
}


// fn calculate_bmi() {
 
//    println!("enter your weight in kg: ");
//     let mut weight_str = String::new();
//     io::stdin().read_line(&mut weight_str).expect("failed to read line");
//     let _weight: f32 = weight_str.trim().parse().expect("please type a number!");


//     println!("enter your height in cm: ");
//     let mut height_str = String::new();
//     io::stdin().read_line(&mut height_str).expect("failed to read line");
//     let _height: f32 = height_str.trim().parse().expect("please type a number!");

//     let height_m = _height / 100.0;
//     let bmi = _weight / (height_m * height_m);
//     println!("your bmi is: {:.2}", bmi);
// }
 
// fn random_number() -> String {

//     println!("tossing a coin...");
//     let coin: i32 = rand::thread_rng().gen_range(0..2);
//     let result: String = if coin == 0 {
//         String::from("heads")
//     } else {
//         String::from("tails")
//     };
//     match result.as_str() {
//         "heads" => println!("you got heads"),
//         "tails" => println!("you got tails"),
//         _ => println!("error"),
//     }
//     result
// }
