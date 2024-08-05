use std::io;
use dialoguer::{theme::ColorfulTheme, Confirm};

fn main() {
    let mut encrepted_message = String::new();

    // Ask the user if they want to encrypt letters to numbers
    let letter_to_num = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to encrypt letters to numbers?")
        .default(false) // Default to "No"
        .interact()
        .unwrap();

    if letter_to_num == true {
        println!("You chose to encrypt letters to numbers.");
    } else {
        println!("You chose not to encrypt letters to numbers.");
    }

    //Get the message from user
    println!("Whats the message");
    let mut unencrepted_message = String::new();
    io::stdin()
        .read_line(&mut unencrepted_message)
        .expect("Failed to read input");
    //Remove the line_end symbol at the end of the message
    unencrepted_message = unencrepted_message.trim_end().to_string();
    // if letter to num is true check if the message has any numbers
    if letter_to_num == true {
        for char in unencrepted_message.chars() {
            if char.is_digit(10) {
                panic!("Please dont enter a number if you are encrypting letters to numbers");
            }
        }
    }

    //Get the shift from user 
    println!("Whats the shift");
    let mut shift = String::new();
    io::stdin()
        .read_line(&mut shift)
        .expect("Failed to read input");
    //Convert String to u8 and check if its a number
    let shift_num: u8 = match shift.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please enter a number thats less than 255");
        }
    };
  
    //Encrept the message
    for char in unencrepted_message.chars() {
        //Find if the character is lower/upercase or a number
        let mut symbol = false;
        let mut max_size = b'z';
        let mut size_length: u8 = 0;
        match char as u8 {
            b'a'..=b'z' => {
                max_size = b'z';
                size_length = 26;
            },
            b'A'..=b'Z' => {
                max_size = b'Z';
                size_length = 26; 
            },
            b'0'..=b'9' => {
                max_size = b'9';
                size_length = 10; 
            },
            _ => {
                symbol = true;
            }
        }
        //Shift the character by the shift amount
        let mut ascii_value;
        if letter_to_num == true {
            ascii_value = char as u8 - 48 + shift_num;
            max_size = b'9';
            size_length = 10;
        } else {
            ascii_value = char as u8 + shift_num;
        }

        //If the shifted character goes past the maximum amount it should be
        //Wrap around to the start
        loop {
            if ascii_value > max_size {
                ascii_value -= size_length
            } else {
                break;
            };
        };
        if symbol == false {
            encrepted_message.push(ascii_value as char);
        } else {
            encrepted_message.push(char);
        }
    } 
    println!("Encrepted Message: {}", encrepted_message);
}
