pub mod enums;

use enums::InputOption;
use std::io;

#[derive(Debug)]
pub struct UserInput {}

impl UserInput {
    pub fn prompt_user_input(message: &String, is_numeric: bool) -> InputOption {
        println!("{}", message);
        let mut value: String = String::new();

        Self::read_stdin(&mut value);

        match is_numeric {
            false => InputOption::Text(value.trim().to_string()),
            true => {
                let value: i32 = value.trim().parse().expect("Expected numeric input");
                InputOption::Numeric(value)
            }
        }
    }

    fn read_stdin(value: &mut String) -> () {
        io::stdin()
            .read_line(value)
            .expect("Failed to take user input: ");
    }
}
