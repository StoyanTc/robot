use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub trait Face {}

// Define types for each direction
#[derive(Clone, ToSchema)]
pub struct North;

#[derive(Clone, ToSchema)]
pub struct East;

#[derive(Clone, ToSchema)]
pub struct South;

#[derive(Clone, ToSchema)]
pub struct West;

impl Face for North {}
impl Face for East {}
impl Face for South {}
impl Face for West {}

// Define type aliases for each robot direction
pub type NorthRobot = Robot<North>;
pub type EastRobot = Robot<East>;
pub type SouthRobot = Robot<South>;
pub type WestRobot = Robot<West>;

// Generic Robot struct
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Robot<S: Face> {
    pub position: Position,

    #[serde(skip)]
    pub facing: std::marker::PhantomData<S>, // PhantomData to represent direction
}

impl<S: Face> Robot<S> {
    pub fn new(x: i32, y: i32) -> Robot<S> {
        Robot {
            position: Position { x, y },
            facing: std::marker::PhantomData,
        }
    }

    #[cfg(test)]
    pub fn is_facing<U: 'static>(&self) -> bool
    where
        S: 'static,
    {
        std::any::TypeId::of::<S>() == std::any::TypeId::of::<U>()
    }
}

impl Robot<North> {
    pub fn turn_left(self) -> Robot<West> {
        Robot {
            position: self.position,
            facing: std::marker::PhantomData,
        }
    }

    pub fn turn_right(self) -> Robot<East> {
        Robot {
            position: self.position,
            facing: std::marker::PhantomData,
        }
    }

    pub fn advance(mut self) -> Self {
        self.position.y += 1;
        self
    }
}

impl Robot<East> {
    pub fn turn_left(self) -> Robot<North> {
        Robot {
            position: self.position,
            facing: std::marker::PhantomData,
        }
    }

    pub fn turn_right(self) -> Robot<South> {
        Robot {
            position: self.position,
            facing: std::marker::PhantomData,
        }
    }

    pub fn advance(mut self) -> Self {
        self.position.x += 1;
        self
    }
}

impl Robot<South> {
    pub fn turn_left(self) -> Robot<East> {
        Robot {
            position: self.position,
            facing: std::marker::PhantomData,
        }
    }

    pub fn turn_right(self) -> Robot<West> {
        Robot {
            position: self.position,
            facing: std::marker::PhantomData,
        }
    }

    pub fn advance(mut self) -> Self {
        self.position.y -= 1;
        self
    }
}

impl Robot<West> {
    pub fn turn_left(self) -> Robot<South> {
        Robot {
            position: self.position,
            facing: std::marker::PhantomData,
        }
    }

    pub fn turn_right(self) -> Robot<North> {
        Robot {
            position: self.position,
            facing: std::marker::PhantomData,
        }
    }

    pub fn advance(mut self) -> Self {
        self.position.x -= 1;
        self
    }
}

// Create an enum that can hold any of the typed robots
#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub enum RobotWithFace {
    North(NorthRobot),
    East(EastRobot),
    South(SouthRobot),
    West(WestRobot),
}

impl RobotWithFace {
    pub fn _new(x: i32, y: i32, facing: &str) -> RobotWithFace {
        match facing {
            "North" => RobotWithFace::North(Robot::new(x, y)),
            "East" => RobotWithFace::East(Robot::new(x, y)),
            "South" => RobotWithFace::South(Robot::new(x, y)),
            "West" => RobotWithFace::West(Robot::new(x, y)),
            _ => panic!("Invalid facing direction"),
        }
    }

    pub fn turn_left(&mut self) {
        match self {
            RobotWithFace::North(robot) => *self = RobotWithFace::West(robot.clone().turn_left()),
            RobotWithFace::East(robot) => *self = RobotWithFace::North(robot.clone().turn_left()),
            RobotWithFace::South(robot) => *self = RobotWithFace::East(robot.clone().turn_left()),
            RobotWithFace::West(robot) => *self = RobotWithFace::South(robot.clone().turn_left()),
        }
    }

    pub fn turn_right(&mut self) {
        match self {
            RobotWithFace::North(robot) => *self = RobotWithFace::East(robot.clone().turn_right()),
            RobotWithFace::East(robot) => *self = RobotWithFace::South(robot.clone().turn_right()),
            RobotWithFace::South(robot) => *self = RobotWithFace::West(robot.clone().turn_right()),
            RobotWithFace::West(robot) => *self = RobotWithFace::North(robot.clone().turn_right()),
        }
    }

    pub fn advance(&mut self) {
        match self {
            RobotWithFace::North(robot) => *self = RobotWithFace::North(robot.clone().advance()),
            RobotWithFace::East(robot) => *self = RobotWithFace::East(robot.clone().advance()),
            RobotWithFace::South(robot) => *self = RobotWithFace::South(robot.clone().advance()),
            RobotWithFace::West(robot) => *self = RobotWithFace::West(robot.clone().advance()),
        }
    }
}

pub fn run() {
    println!("Running Type State Pattern Solution!");
}

#[cfg(test)]
mod test {

    use crate::solutions::type_state_pattern::{East, North, Robot};

    #[test]
    fn test_robot() {
        let robot = Robot::<North>::new(7, 3);
        let robot = robot.turn_right();
        assert!(robot.is_facing::<East>());
        let robot = robot.advance();
        assert_eq!(robot.position.x, 8);
        let robot = robot.advance();
        assert_eq!(robot.position.x, 9);
        let robot = robot.turn_left();
        assert!(robot.is_facing::<North>());
        let robot = robot.advance();
        assert_eq!(robot.position.y, 4);
    }
}
/*
fn main() {
    let robot = Robot::new(3, 8); // Starts facing North at (3,8)
    let robot = robot.turn_right().advance().turn_right().advance(); // Moves East and then South
    println!("{:?}", robot.position); // Should be (4,7)
}
 */
