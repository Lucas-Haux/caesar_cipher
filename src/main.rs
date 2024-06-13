use std::io;

fn main() {
    let mut encrepted_message = String::new();

    //Get the message from user
    println!("Whats the message");
    let mut unencrepted_message = String::new();
    io::stdin()
        .read_line(&mut unencrepted_message)
        .expect("Failed to read input");

    //Get the shift from user 
    println!("Whats the shift");
    let mut shift = String::new();
    io::stdin()
        .read_line(&mut shift)
        .expect("Failed to read input");
    let shift_num: u8 = match shift.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please enter a valid integer");
        }
    };

    //Encrept the message
    for char in unencrepted_message.chars() {
        //Find if the character is lower/upercase or a number
        let mut max_size = b'z';
        let mut size_length: u8 = 0;
        match char as u8 {
            b'a'..=b'z' => {
                max_size = b'z';
                size_length = 26;
                println!("lower");
            },
            b'A'..=b'Z' => {
                max_size = b'Z';
                size_length = 26; 
                println!("upper");
            },
            b'0'..=b'9' => {
                max_size = b'9';
                size_length = 10; 
                println!("number");
            },
            _ => {
                println!("It's something else");
            }
        }
        //Shift the character by the shift amount
        let ascii_value = char as u8 + shift_num;
        //If the shifted character goes past the maximum amount it should be
        //Wrap around to the start
        let shifted_char = if ascii_value <= max_size {
            ascii_value as char
        } else {
            (ascii_value - size_length) as char
        };
        encrepted_message.push(shifted_char);
    } 
    println!("Encrepted Message: {}", encrepted_message);
}
