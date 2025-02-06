use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing, Json, Router,
};
use std::net::SocketAddr;

use simulation::SimulationInput;
use crate::simulation::{simulation::run_simulations, templates::get_templates};

mod simulation;




#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/test", routing::post(simulation_handler))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::POST]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Response {
    successfull_simulations: f32,
    average_pulls_left: u32,
}

async fn simulation_handler(Json(inputs): Json<SimulationInput>) -> Json<Response> {
    let result = run_simulations(
        &mut inputs.clone(),
        get_templates().get("hsr").expect("Requested template missing")
    );

    Json::from(Response {
        successfull_simulations:  result.0,
        average_pulls_left: result.1
    })
}