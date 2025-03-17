use crate::controller::{move_robot, reposition_robot, reset_robot, robot_position, ApiDoc};
use actix_web::{web, App, HttpServer};
use controller::RobotState;
use solutions::*;
use utoipa::OpenApi;

use std::sync::Mutex;
use utoipa_swagger_ui::SwaggerUi;

mod controller;
mod solutions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(not(feature = "type_state"))]
    let robot_state = web::Data::new(RobotState(Mutex::new(Robot::new(0, 0, Direction::North))));

    #[cfg(feature = "type_state")]
    let robot_state = web::Data::new(RobotState(Mutex::new(RobotWithFace::North(Robot::new(
        0, 0,
    )))));

    solutions::run();

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(robot_state.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/move_robot", web::post().to(move_robot))
            .route("/reposition_robot", web::post().to(reposition_robot))
            .route("/reset_robot", web::post().to(reset_robot))
            .route("/robot_position", web::get().to(robot_position))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod test {
    use crate::{Direction, Robot};

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
