

fn main() {
    
    let mut number: Vec<i32> = Vec::new();
    println!("hit 0 to exit, or enter a number to add to the list");
    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if input == 0 {
            break;
        }
        
        number.push(input);
    }

    let mut even_count: i32 = 0;
    let mut index_number: usize = 0;
    while index_number < number.len() {
        if number[index_number] % 2 == 0 {
            even_count += 1;
        }
        index_number += 1;
    }
    let mut summ:i32 = 0;
    for num in &number {
        summ += num;
    }
    println!("The sum of the numbers is: {}", summ);
    println!("You entered {} even numbers", even_count);
    println!("You entered: {:?}", number);

}
