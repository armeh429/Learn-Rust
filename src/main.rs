use std::ops::Range;
fn main() {
 

 for_loop_example();


}           

fn for_loop_example() {
    let rang1 :Range<i32> = 1..10;
    for i in rang1 {
        println!("The value is: {}", i);
    }   
} 