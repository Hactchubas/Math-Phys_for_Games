use std::path::PathBuf;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
mod structs;
use structs::vector::Vector;

#[get("/")]
async fn app_home() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open(PathBuf::from(
        "./static/index.html",
    ))?)
}


// Configuração das rotas
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
    )
    .service(app_home);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Abertura da página no navegador
    println!("Starting server at http://127.0.0.1:8080");
    if let Err(e) = webbrowser::open("http://127.0.0.1:8080") {
        println!("Failed to open browser: {}", e);
    }

    // Configuração do servidor
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Deserialize)]
struct VectorOperationRequest {
    v1: Vector,
    v2: Vector,
    scalar: Option<f64>,
}
#[derive(Deserialize)]
struct VectorReactionRequest {
    v1: Vector,
    v2: Vector,
    alfa: f64,
    beta: f64,
}
#[derive(Deserialize)]
struct LineSegmentsIntersectionRequest {
    segment_a: (Vector, Vector),
    segment_b: (Vector, Vector),
}
#[derive(Deserialize)]
struct LineSegmentsNormalRequest {
    segment: (Vector, Vector),
}
