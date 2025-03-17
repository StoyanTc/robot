## Robot Simulator

### Description
This is a simple robot simulator in Rust with a REST interface.
The robots have three possible movements:
  - turn right
  - turn left
  - advance

Robots are placed on a hypothetical infinite grid, 
facing a particular direction (north, east, south, or west) at a set of {x,y} coordinates, e.g., {3,8}, 
with coordinates increasing to the north and east.

### Example
The robot then receives a number of instructions, at which point the testing facility verifies the robot's new position, and in which direction it is pointing.

  - The letter-string "RAALAL" means: 
    - Turn right
    - Advance twice
    - Turn left
    - Advance once
    - Turn left yet again
  - Say a robot starts at {7, 3} facing north. Then running this stream of instructions should leave it at {9, 4} facing west.

### Solutions

The simulator is implemented by different approaches and patterns.

- Command pattern
- State pattern
- Type state pattern
- Non-pattern

However only for `type state pattern` and `no pattern` is implemented REST API with OpenAPI UI.

### Build 

- With no pattern (default)
```
cargo build
```
- With type state pattern

```
 cargo build --no-default-features --features type_state 
```

### Run

- With no pattern (default)
```
cargo run
```
- With type state pattern

```
 cargo run --no-default-features --features type_state 
```

### Explore OpenAPI UI
Open in browser the url `http://127.0.0.1:8080/`.



  



