mod submarine;
mod user_input;

use submarine::Submarine;
use user_input::enums::InputOption;
use user_input::UserInput;

pub const ALL_DIRECTIONS: [&str; 4] = ["up", "down", "forward", "backward"];

fn start_game(submarine: &mut Submarine) {
    loop {
        // Input messages
        let directions_msg: String = format!("Select a direction from: {:?}:", ALL_DIRECTIONS);
        let steps_msg: String = format!("How many steps?:");

        // Prompt messages at stdin
        let direction: InputOption = UserInput::prompt_user_input(&directions_msg, false);
        let steps = UserInput::prompt_user_input(&steps_msg, true);

        // Remap user input
        let direction: String = match direction {
            InputOption::Text(value) => value,
            _ => panic!("Received unexpected InputOption as direction"),
        };

        let steps: i32 = match steps {
            InputOption::Numeric(value) => value,
            _ => panic!("Received unexpected InputOption as steps"),
        };

        Submarine::validate_direction(&String::from(&direction));
        submarine.calc_new_position(direction, steps);
    }
}

fn main() {
    let mut submarine = Submarine::new();
    println!("Your submarine is at: {:?}", submarine.position);

    start_game(&mut submarine);
}
