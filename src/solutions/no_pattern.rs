use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ToSchema)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, facing: Direction) -> Robot {
        Robot { x, y, facing }
    }

    pub fn execute(&mut self, instruction: char) {
        match instruction {
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            'A' => self.advance(),
            _ => (),
        }
    }

    pub fn turn_left(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn advance(&mut self) {
        (self.x, self.y) = match self.facing {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        }
    }
}

pub fn run() {
    println!("Running Default (Non-Pattern) Solution!");
}

#[cfg(test)]
mod test {
    use crate::solutions::no_pattern::{Direction, Robot};

    #[test]
    fn test_robot() {
        let mut robot = Robot::new(7, 3, Direction::North);
        robot.execute('R');
        assert_eq!(robot.facing, Direction::East);
        robot.execute('A');
        assert_eq!(robot.x, 8);
        robot.execute('A');
        assert_eq!(robot.x, 9);
        robot.execute('L');
        assert_eq!(robot.facing, Direction::North);
        robot.execute('A');
        assert_eq!(robot.y, 4);
    }
}
