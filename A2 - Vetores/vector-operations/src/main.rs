

use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
mod structs;
use structs::vector::Vector;

// Endpoint para soma de vetores
#[post("/soma")]
async fn soma_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.clone() + data.v2.clone();
    HttpResponse::Ok().json(resultado)
}

// Endpoint para subtração de vetores
#[post("/subtracao")]
async fn subtracao_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.clone() - data.v2.clone();
    HttpResponse::Ok().json(resultado)
}

// Endpoint para multiplicação por escalar
#[post("/redimensionar")]
async fn redimensionar(data: web::Json<VectorOperationRequest>) -> impl Responder {
    if let Some(k) = data.scalar {
        let resultado = data.v1.scale(k);
        HttpResponse::Ok().json(resultado)
    } else {
        HttpResponse::BadRequest().body("Escalar não fornecido")
    }
}

// Endpoint para produto escalar
#[post("/produto-escalar")]
async fn produto_escalar(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.dot_product(&data.v2);
    HttpResponse::Ok().json(resultado)
}

// Endpoint para produto vetorial
#[post("/produto-vetorial")]
async fn produto_vetorial(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.cross_product(&data.v2);
    HttpResponse::Ok().json(resultado)
}

// Endpoint para projeção
#[post("/projecao")]
async fn projecao_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.projected_at(&data.v2);
    HttpResponse::Ok().json(resultado)
}

// Endpoint para reflexão
#[post("/reflexao")]
async fn reflexao_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.reflected_at(&data.v2);
    HttpResponse::Ok().json(resultado)
}

// Configuração das rotas
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(soma_vetores)
        .service(subtracao_vetores)
        .service(redimensionar)
        .service(produto_escalar)
        .service(produto_vetorial)
        .service(projecao_vetores)
        .service(reflexao_vetores);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");  
    if let Err(e) = webbrowser::open("http://127.0.0.1:8080/vector-visualization.html") {
        println!("Failed to open browser: {}", e);
    }

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
