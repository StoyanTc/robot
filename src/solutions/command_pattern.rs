#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

// The Robot (receiver)
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, facing: Direction) -> Robot {
        Robot { x, y, facing }
    }

    fn turn_left(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn advance(&mut self) {
        match self.facing {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }
}

// Command trait
trait Command {
    fn execute(&self, robot: &mut Robot);
    fn undo(&self, robot: &mut Robot);
}

// Concrete commands
struct TurnLeftCommand;
struct TurnRightCommand;
struct AdvanceCommand;

impl Command for TurnLeftCommand {
    fn execute(&self, robot: &mut Robot) {
        robot.turn_left();
    }

    fn undo(&self, robot: &mut Robot) {
        robot.turn_right(); // Opposite of turn_left
    }
}

impl Command for TurnRightCommand {
    fn execute(&self, robot: &mut Robot) {
        robot.turn_right();
    }

    fn undo(&self, robot: &mut Robot) {
        robot.turn_left(); // Opposite of turn_right
    }
}

impl Command for AdvanceCommand {
    fn execute(&self, robot: &mut Robot) {
        robot.advance();
    }

    fn undo(&self, robot: &mut Robot) {
        // Move in opposite direction by rotating 180° temporarily
        robot.turn_right();
        robot.turn_right();
        robot.advance();
        robot.turn_right();
        robot.turn_right();
    }
}

// Command invoker that maintains history
struct RobotController {
    history: Vec<Box<dyn Command>>,
    robot: Robot,
}

impl RobotController {
    fn new(x: i32, y: i32, facing: Direction) -> Self {
        Self {
            history: Vec::new(),
            robot: Robot::new(x, y, facing),
        }
    }

    fn execute(&mut self, command: Box<dyn Command>) {
        command.execute(&mut self.robot);
        self.history.push(command);
    }

    fn undo_last(&mut self) {
        if let Some(command) = self.history.pop() {
            command.undo(&mut self.robot);
        }
    }

    fn process_instruction(&mut self, instruction: char) {
        match instruction {
            'L' => self.execute(Box::new(TurnLeftCommand)),
            'R' => self.execute(Box::new(TurnRightCommand)),
            'A' => self.execute(Box::new(AdvanceCommand)),
            _ => println!("Unknown command: {}", instruction),
        }
    }

    fn position(&self) -> (i32, i32, &Direction) {
        (self.robot.x, self.robot.y, &self.robot.facing)
    }
}

pub fn run() {
    println!("Running Command Pattern Solution!");
}

#[cfg(test)]
mod test {
    use crate::solutions::command_pattern::{Direction, RobotController};

    #[test]
    fn test_robot() {
        let mut robot_controller = RobotController::new(7, 3, Direction::North);
        robot_controller.process_instruction('R');
        assert_eq!(robot_controller.robot.facing, Direction::East);
        robot_controller.process_instruction('A');
        assert_eq!(robot_controller.robot.x, 8);
        robot_controller.process_instruction('A');
        assert_eq!(robot_controller.robot.x, 9);
        robot_controller.process_instruction('L');
        assert_eq!(robot_controller.robot.facing, Direction::North);
        robot_controller.process_instruction('A');
        assert_eq!(robot_controller.robot.y, 4);
    }
}
