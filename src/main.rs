/* This program is intended to be a random number generator
 * The idea is to be able to generate random numbers, passwords,
 * and dice rolls based on the request from a client*/
mod app;
mod rest;
use color_eyre::eyre::Report;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use rest::Api;

#[tokio::main]
async fn main() -> Result<(), Report> {
    let api_service =
        OpenApiService::new(Api, "Random Number Gen", "1.0").server("http://localhost:5150/rand");
    let ui = api_service.swagger_ui();
    Server::new(TcpListener::bind("0.0.0.0:5150"))
        .run(Route::new().nest("/rand", api_service).nest("/", ui))
        .await?;

    Ok(())
}
