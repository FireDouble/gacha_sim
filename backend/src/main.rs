use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing, Json, Router,
};
use tokio::net::TcpListener;

use logic::{calculation, simulation::simulate, Inputs, SimulationOutput};

mod logic;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/simulate", routing::post(simulation_handler))
        .route("/test", routing::get(test_handler))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE])
                .allow_methods([Method::POST]),
        );

    let listener = TcpListener::bind("0.0.0.0:3030").await.unwrap();

    println!("Server started at port 3030");

    axum::serve(listener, app).await.unwrap();
}

async fn simulation_handler(Json(inputs): Json<Inputs>) -> Json<SimulationOutput> {
    Json::from(simulate(&mut inputs.clone()))
}
async fn test_handler() -> Json<String> {
    Json::from("Hello World!".to_string())
}
