use std::sync::Arc;

// Direction trait defining behavior for each state
trait Direction {
    fn turn_right(self: Arc<Self>) -> Arc<dyn Direction>;
    fn turn_left(self: Arc<Self>) -> Arc<dyn Direction>;
    fn advance(&self, x: i32, y: i32) -> (i32, i32);
    fn name(&self) -> &'static str;
}

// Concrete direction states
struct North;
struct East;
struct South;
struct West;

// Implementation for North state
impl Direction for North {
    fn turn_right(self: Arc<Self>) -> Arc<dyn Direction> {
        Arc::new(East)
    }

    fn turn_left(self: Arc<Self>) -> Arc<dyn Direction> {
        Arc::new(West)
    }

    fn advance(&self, x: i32, y: i32) -> (i32, i32) {
        (x, y + 1)
    }

    fn name(&self) -> &'static str {
        "NORTH"
    }
}

// Implementation for East state
impl Direction for East {
    fn turn_right(self: Arc<Self>) -> Arc<dyn Direction> {
        Arc::new(South)
    }

    fn turn_left(self: Arc<Self>) -> Arc<dyn Direction> {
        Arc::new(North)
    }

    fn advance(&self, x: i32, y: i32) -> (i32, i32) {
        (x + 1, y)
    }

    fn name(&self) -> &'static str {
        "EAST"
    }
}

// Implementation for South state
impl Direction for South {
    fn turn_right(self: Arc<Self>) -> Arc<dyn Direction> {
        Arc::new(West)
    }

    fn turn_left(self: Arc<Self>) -> Arc<dyn Direction> {
        Arc::new(East)
    }

    fn advance(&self, x: i32, y: i32) -> (i32, i32) {
        (x, y - 1)
    }

    fn name(&self) -> &'static str {
        "SOUTH"
    }
}

// Implementation for West state
impl Direction for West {
    fn turn_right(self: Arc<Self>) -> Arc<dyn Direction> {
        Arc::new(North)
    }

    fn turn_left(self: Arc<Self>) -> Arc<dyn Direction> {
        Arc::new(South)
    }

    fn advance(&self, x: i32, y: i32) -> (i32, i32) {
        (x - 1, y)
    }

    fn name(&self) -> &'static str {
        "WEST"
    }
}

// Robot struct holding position and current direction state
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub facing: Arc<dyn Direction>,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Arc<dyn Direction>) -> Robot {
        Robot {
            x,
            y,
            facing: direction,
        }
    }

    fn execute(&mut self, movement: char) {
        match movement {
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            'A' => self.advance(),
            _ => (),
        }
    }

    fn turn_right(&mut self) {
        self.facing = self.facing.clone().turn_right();
    }

    fn turn_left(&mut self) {
        self.facing = self.facing.clone().turn_left();
    }

    fn advance(&mut self) {
        let (new_x, new_y) = self.facing.advance(self.x, self.y);
        self.x = new_x;
        self.y = new_y;
    }
}

pub fn run() {
    println!("Running State Pattern Solution!");
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::solutions::state_pattern::{East, North, Robot};

    #[test]
    fn test_robot() {
        let mut robot = Robot::new(7, 3, Arc::new(North));
        robot.execute('R');
        assert_eq!(robot.facing, East);
        robot.execute('A');
        assert_eq!(robot.x, 8);
        robot.execute('A');
        assert_eq!(robot.x, 9);
        robot.execute('L');
        assert_eq!(robot.facing, North);
        robot.execute('A');
        assert_eq!(robot.y, 4);
    }
}

/* fn main() {
    let instruction = "RAALAL";
    let mut robot = Robot::new(7, 3, Rc::new(North));

    for movement in instruction.chars() {
        robot.execute(movement);
    }

    println!(
        "Robot is at ({}, {}) facing {}",
        robot.x,
        robot.y,
        robot.direction.name()
    );
} */
