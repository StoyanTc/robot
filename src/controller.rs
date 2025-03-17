use std::sync::Mutex;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::solutions::*;

#[cfg(feature = "no_pattern")]
pub struct RobotState(pub Mutex<Robot>);

#[cfg(feature = "type_state")]
pub struct RobotState(pub Mutex<RobotWithFace>);

#[derive(Deserialize, ToSchema)]
pub struct MoveInstruction {
    pub instructions: String,
}

/// Move the robot based on a series of instructions (`L`, `R`, `A`).
#[cfg(feature = "no_pattern")]
#[utoipa::path(
    post,
    path = "/move_robot",
    request_body = MoveInstruction,
    responses(
        (status = 200, description = "Robot moved successfully", body = Robot)
    )
)]
pub async fn move_robot(
    data: web::Data<RobotState>,
    req: web::Json<MoveInstruction>,
) -> impl Responder {
    let mut robot = data.0.lock().unwrap();
    for movement in req.instructions.chars() {
        robot.execute(movement);
    }
    HttpResponse::Ok().json(robot.clone())
}

#[cfg(feature = "type_state")]
#[utoipa::path(
    post,
    path = "/move_robot",
    request_body = MoveInstruction,
    responses(
        (status = 200, description = "Robot facing", body = RobotWithFace),
    )
)]
pub async fn move_robot(
    data: web::Data<RobotState>,
    req: web::Json<MoveInstruction>,
) -> impl Responder {
    let mut robot = data.0.lock().unwrap();
    for movement in req.instructions.chars() {
        match movement {
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            _ => (),
        }
    }
    HttpResponse::Ok().json(robot.clone())
}

/// Set the robot's position manually.
#[cfg(feature = "no_pattern")]
#[utoipa::path(
    post,
    path = "/reposition_robot",
    request_body = Robot,
    responses(
        (status = 200, description = "Robot repositioned successfully", body = Robot)
    )
)]
pub async fn reposition_robot(
    data: web::Data<RobotState>,
    req: web::Json<Robot>,
) -> impl Responder {
    let mut robot = data.0.lock().unwrap();
    robot.x = req.x;
    robot.y = req.y;
    robot.facing = req.facing.clone();
    HttpResponse::Ok().json(robot.clone())
}

#[cfg(feature = "type_state")]
#[utoipa::path(
    post,
    path = "/reposition_robot",
    request_body = RobotWithFace,
    responses(
        (status = 200, description = "Robot facing", body = RobotWithFace),
    )
)]
pub async fn reposition_robot(
    data: web::Data<RobotState>,
    req: web::Json<RobotWithFace>,
) -> impl Responder {
    let mut robot = data.0.lock().unwrap();
    *robot = req.clone();
    HttpResponse::Ok().json(robot.clone())
}

/// Reset the robot to its initial position.
#[cfg(feature = "no_pattern")]
#[utoipa::path(
    post,
    path = "/reset_robot",
    responses(
        (status = 200, description = "Robot reset successfully", body = Robot)
    )
)]
pub async fn reset_robot(data: web::Data<RobotState>) -> impl Responder {
    let mut robot = data.0.lock().unwrap();
    *robot = Robot {
        x: 0,
        y: 0,
        facing: Direction::North,
    };

    HttpResponse::Ok().json(robot.clone())
}

#[cfg(feature = "type_state")]
#[utoipa::path(
    post,
    path = "/reset_robot",
    responses(
        (status = 200, description = "Robot facing North", body = RobotWithFace),
    )
)]
pub async fn reset_robot(data: web::Data<RobotState>) -> impl Responder {
    let mut robot = data.0.lock().unwrap();
    *robot = RobotWithFace::North(Robot::new(0, 0));

    HttpResponse::Ok().json(robot.clone())
}

/// Get the robot's current position and direction.
#[cfg(feature = "no_pattern")]
#[utoipa::path(
    get,
    path = "/robot_position",
    responses(
        (status = 200, description = "Current robot position", body = Robot)
    )
)]
pub async fn robot_position(data: web::Data<RobotState>) -> impl Responder {
    let robot = data.0.lock().unwrap();
    HttpResponse::Ok().json(robot.clone())
}

#[cfg(feature = "type_state")]
#[utoipa::path(
    get,
    path = "/robot_position",
    responses(
        (status = 200, description = "Robot facing North", body = RobotWithFace),
    )
)]
pub async fn robot_position(data: web::Data<RobotState>) -> impl Responder {
    let robot = data.0.lock().unwrap();
    HttpResponse::Ok().json(robot.clone())
}

/// OpenAPI documentation setup.
#[cfg(feature = "no_pattern")]
#[derive(OpenApi)]
#[openapi(
    paths(move_robot, reposition_robot, reset_robot, robot_position),
    components(schemas(Robot, MoveInstruction, Direction))
)]
pub struct ApiDoc;

#[cfg(feature = "type_state")]
#[derive(OpenApi)]
#[openapi(
    paths(move_robot, reposition_robot, reset_robot, robot_position),
    components(schemas(RobotWithFace, MoveInstruction))
)]
pub struct ApiDoc;
