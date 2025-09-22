

fn main() {
    let mut number: [u8; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", number);
    println!("First element: {}", number[4]);
    let mut number2: [u8; 5] = [3; 5];
    println!("Array2: {:?}", number2);
    number2[1] = 4;
    println!("Array2: {:?}", number2);
    for i in 0..5 {
        number[i] = (i as u8) * 5;
    }
    println!("Array: {:?}", number);

    match number2.get(4) {
        Some(x) => println!("The tenth element is {}", x),
        None => panic!("There is no tenth element.")
    }

    println!("{:?}",number2.iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join(", "));
}
