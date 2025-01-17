#![allow(non_snake_case)]

use std::io;

pub fn get_u8_input(message: &str) -> u8 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<u8>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_u32_input(message: &str) -> u32 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<u32>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_u64_input(message: &str) -> u64 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<u64>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_u128_input(message: &str) -> u128 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<u128>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_usize_input(message: &str) -> usize {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<usize>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_i8_input(message: &str) -> i8 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<i8>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_i32_input(message: &str) -> i32 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<i32>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_i64_input(message: &str) -> i64 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<i64>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_i128_input(message: &str) -> i128 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<i128>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_isize_input(message: &str) -> isize {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<isize>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_f32_input(message: &str) -> f32 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<f32>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_f64_input(message: &str) -> f64 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<f64>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_bool_input(message: &str) -> bool {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<bool>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_char_input(message: &str) -> char {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<char>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 

pub fn get_string_input(message: &str) -> String {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<String>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("\nInvalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 