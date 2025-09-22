use std::ops::Range;

fn main() {
    let rang_1: Range<i32> = 1..5;
    for num in rang_1 {
        println!("{}", num);
    }
}
