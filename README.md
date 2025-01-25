Input-handler is a basic library designed to handle user inputs of any type and avoid a panic. It has error handling built in

# Current functions
### get_u8_input
### get_u32_input
### get_u64_input
### get_u128_input
### get_usize_input
### get_i8_input
### get_i32_input
### get_i64_input
### get_i128_input
### get_isize_input
### get_f32_input
### get_f64_input
### get_bool_input
### get_char_input
### get_string_input

# Usage
```rust
get_u8_input(message);
```
Where message is what the user will be asked
# How it works

Each function follows the same principles, adjusting for each data type. Here is the code for get_u32_input:

```rust
pub fn get_u32_input(message: &str) -> u32 {
    println!("{}", message);
    let mut userInput = String::new();
    loop {
        io::stdin().read_line(&mut userInput).expect("Failed to read line");
        match userInput.trim().parse::<u32>() {
            Ok(value) => {return value;}
            Err(e) => {
                println!("Invalid input! Error: {}", e);
                userInput = String::new();
            }
        }
    }
} 
```

The function takes in &str data "message", which is the message displayed before the input. The input is then read and checked if it can be converted to u32. If it can, the value is returned, if not an error is displayed and the input variable is reset then a new input is gathered.
