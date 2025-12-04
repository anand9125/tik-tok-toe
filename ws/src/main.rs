use actix::Actor;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, body, web};
use actix_web_actors::ws;
use uuid::Uuid;
use std::io::Result;


pub mod actors;
pub use actors::*;
pub mod state;
pub use state::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .route("/ws", web::get().to(ws_route))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}





