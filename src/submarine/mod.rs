use super::ALL_DIRECTIONS;

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Submarine {
    pub position: Point,
}

impl Submarine {
    pub fn new() -> Self {
        Self {
            position: Point { x: 0, y: 0 },
        }
    }

    pub fn calc_new_position(&mut self, direction: String, steps: i32) -> &Self {
        match direction.as_str() {
            "down" => self.position.y += steps,
            "up" => self.position.y -= steps,
            "forward" => self.position.x += steps,
            "backward" => self.position.x -= steps,
            _ => panic!("Unexpected direction to calculate"),
        }

        println!(
            "Your submarine has been repositioned at: {:?}",
            self.position
        );
        self
    }

    pub fn validate_direction(direction: &String) -> () {
        match ALL_DIRECTIONS.contains(&direction.as_str()) {
            true => (),
            _ => panic!("direction provided not supported"),
        }
    }
}
