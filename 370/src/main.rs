use std::io;

fn main() {
    println!("Please input a UPC number");
    
    // Allocate on the heap to read unknown input size
    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Failed to read UPC number");
    number = number.trim_end().to_string();

    let final_digit = upc(&number);
    println!("Check Digit is {}", final_digit);
    println!("UPC Code: {}", number + &final_digit);
}

fn upc(upc: &str) -> String {
    // Chars is required as .length() is byte length
    if upc.chars().count() > 11 || 
        upc.chars().any(|c| !c.is_digit(10)) 
    {
        panic!("UPC is not valid.")
    }

    let offset = 11 - upc.chars().count();
    // Need a mutable reference here as we will add to the value in a loop
    let mut sum = 0;

    // Rust for uses an iterator, enumerate iterates on both index and value
    for (i, c) in upc.chars().enumerate() {
        
        println!("Index: {}, I: {}, C: {}", i+offset, i, c);
        // no need to error check here as we checked that all are valid base 10 digits above
        let digit = c.to_digit(10).unwrap();
        // 1st, 3rd, 5th
        if ((i + offset) % 2) == 0 {
            sum += digit * 3;
        } else {
            sum += digit;
        }
    }
    let result = sum % 10;

    if result == 0  {
        return "0".to_owned()
    } else {
        (10 - result).to_string() 
    }
}