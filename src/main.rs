
fn main(){

    let mut counter:i32 = 0;

    loop {
        if counter >= 10 {
            break;
        } else {
            println!("Counter: {}", counter);
            counter += 1;
        }
       
    }

}
