use std::{iter::Chain, path::PathBuf};

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
mod structs;
use structs::{
    bounding_boxes::{Bounding, BoundingType, Sphere, AABB, OBB},
    elements::LineSegment,
    vector::Vector,
};

#[get("/")]
async fn app_home() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PathBuf::from(
        "./static/html/index.html",
    ))?)
}

// Endpoint para soma de vetores
#[post("/soma")]
async fn soma_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = &data.v1 + &&data.v2;
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
        let resultado = &data.v1 * k;
        HttpResponse::Ok().json(resultado)
    } else {
        HttpResponse::BadRequest().body("Escalar não fornecido")
    }
}

// Endpoint para produto escalar
#[post("/produto-escalar")]
async fn produto_escalar(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.dot_product(&data.v2);
    match resultado {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::BadRequest().body("Não foi possível calcular o produto escalar"),
    }
}

// Endpoint para produto vetorial
#[post("/produto-vetorial")]
async fn produto_vetorial(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.cross_product(&data.v2);
    match resultado {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::BadRequest().body("Não foi possível calcular o produto vetorial"),
    }
}

// Endpoint para projeção
#[post("/projecao")]
async fn projecao_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.projected_at(&data.v2);
    match resultado {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::BadRequest().body("Não foi possível projetar o vetor"),
    }
}

// Endpoint para decomposição
#[post("/decomposicao")]
async fn decomposicao_vetores(data: web::Json<VectorOperationRequest>) -> impl Responder {
    let resultado = data.v1.decompose(&data.v2);
    match resultado {
        Some((v1, v2)) => HttpResponse::Ok().json((v1, v2)),
        None => HttpResponse::BadRequest().body("Não foi possível decompor o vetor"),
    }
}

// Endpoint para reação
#[post("/reacao")]
async fn reacao_vetores(data: web::Json<VectorReactionRequest>) -> impl Responder {
    let resultado = data
        .v1
        .parameterized_reaction(data.alfa, &data.v2, data.beta);
    match resultado {
        Some(r) => HttpResponse::Ok().json(r),
        None => HttpResponse::BadRequest().body("Não foi possível refletir o vetor"),
    }
}

// Endpoint para intersecsão
#[post("/intersecsao")]
async fn intersecsao_segmento(data: web::Json<LineSegmentsIntersectionRequest>) -> impl Responder {
    let segment_a = data.segment_a.0.to_line_segment(&data.segment_a.1);
    let segment_b = data.segment_b.0.to_line_segment(&data.segment_b.1);

    if let Some(_) = segment_a.intersects(&segment_b) {
        HttpResponse::Ok().json(true)
    } else {
        HttpResponse::Ok().json(false)
    }
}

// Endpoint para colisão
#[post("/colisao")]
async fn colisao(data: web::Json<LineSegmentsIntersectionRequest>) -> impl Responder {
    let segment_a = data.segment_a.0.to_line_segment(&data.segment_a.1);
    let segment_b = data.segment_b.0.to_line_segment(&data.segment_b.1);

    if let Some(_) = segment_a.intersects(&segment_b) {
        if let Some(normal) = segment_b.get_normal() {
            let res = segment_a
                .vec_from_seg()
                .parameterized_reaction(1.0, &normal, 1.0);
            match res {
                Some(r) => HttpResponse::Ok().json(r),
                None => HttpResponse::BadRequest().body("Não foi possível refletir o vetor"),
            }
        } else {
            HttpResponse::BadRequest().body("Não foi possível obter a normal do segmento")
        }
    } else {
        HttpResponse::Ok().json(false)
    }
}

// Endpoint para colisão
#[post("/colisao-refletida")]
async fn colisao_refletida(data: web::Json<ReflectedColisionRequest>) -> impl Responder {
    let walls: Vec<LineSegment> = data
        .walls
        .iter()
        .map(|(x, y)| LineSegment::new(x.to_owned(), y.to_owned()))
        .collect();

    let vec_pos = &data.vector.0;
    let vec_vel = &data.vector.1;
    let velocity = LineSegment::new(vec_pos.to_owned(), vec_vel + &vec_pos);

    let intersected: Vec<Option<Vector>> = walls
        .iter()
        .map(|wall| {
            if let Some(_) = velocity.intersects(wall) {
                if let Some(wall_normal) = wall.get_normal() {
                    velocity
                        .vec_from_seg()
                        .parameterized_reaction(1.0, &wall_normal, 1.0)
                        .to_owned()
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    HttpResponse::Ok().json(intersected)
}

// Endpoint para reação
#[post("/normal")]
async fn normal_segmento(data: web::Json<LineSegmentsNormalRequest>) -> impl Responder {
    let seg_vec = data.segment.0.to_owned() - data.segment.1.to_owned();

    if let Some(normal_vec) = seg_vec.normal_vec() {
        HttpResponse::Ok().json(normal_vec.unit())
    } else {
        HttpResponse::BadRequest().body("Não foi possível calcular a normal")
    }
}

// Endpoint para encontrar segmentos que intersectam
#[post("/intersectam")]
async fn segmentos_intersectam(data: web::Json<FindIntersectingRequest>) -> impl Responder {
    let segments = data.segments.to_owned();
    let line_segments: Vec<LineSegment> = segments
        .iter()
        .map(|(x, y)| LineSegment::new(x.to_owned(), y.to_owned()))
        .collect();

    let intersectin_segments: Vec<Option<Vec<Vector>>> = line_segments
        .iter()
        .map(|item| {
            let intersect_vecs: Vec<Vector> = line_segments
                .iter()
                .filter_map(|x| {
                    if let Some(interc_vec) = &x.intersects(&item) {
                        Some(interc_vec.to_owned())
                    } else {
                        None
                    }
                })
                .collect();
            if intersect_vecs.len() > 0 {
                Some(intersect_vecs)
            } else {
                None
            }
        })
        .collect();

    HttpResponse::Ok().json(intersectin_segments)
}

// Endpoint para encontrar angulos entre vetores
#[post("/angulos")]
async fn angulos(data: web::Json<FindAnglesRequest>) -> impl Responder {
    let vectors = data.vectors.to_owned();
    let angles: Vec<Vec<(Result<f64, &str>, Vector)>> = vectors
        .iter()
        .map(|item| {
            vectors
                .iter()
                .filter_map(|x| Some((x.angle_between(&item, 3 as usize), x.to_owned())))
                .collect()
        })
        .collect();

    HttpResponse::Ok().json(angles)
}

// Endpoint para criar bounding volumes
#[post("/envoltorios-construtor")]
async fn envelopes_contructor(data: web::Json<BoundingBoxRequest>) -> impl Responder {
    let bounding_type = data.bounding_type.as_str();
    match BoundingType::from_str(bounding_type) {
        BoundingType::AABB => {
            if let Some(aabb) = AABB::from_points(&data.points) {
                return HttpResponse::Ok().json(aabb);
            } else {
                return HttpResponse::InternalServerError().json("Error");
            }
        }
        BoundingType::OBB => {
            if let Some(obb) = OBB::from_points(&data.points) {
                return HttpResponse::Ok().json(obb);
            } else {
                return HttpResponse::InternalServerError().json("Error");
            }
        }
        BoundingType::Sphere => {
            if let Some(sphere) = Sphere::from_points(&data.points) {
                return HttpResponse::Ok().json(sphere);
            } else {
                return HttpResponse::InternalServerError().json("Error");
            }
        }
        BoundingType::Unknown => {
            HttpResponse::BadRequest().body("Não foi possível construir envoltório")
        }
    }
}

// Endpoint para encontrar envoltórios que intersectam
#[post("/envoltorios-intersectam")]
async fn envelopess_intersectam(data: web::Json<BoundingBoxCollisionRequest>) -> impl Responder {
    let aabb: Vec<(AABB, usize)> = data.aabb.to_owned();
    let obb: Vec<(OBB, usize)> = data.obb.to_owned();
    let sphere: Vec<(Sphere, usize)> = data.sphere.to_owned();

    let boundings: Vec<(usize, Box<dyn Bounding>)> = aabb
        .to_owned()
        .into_iter()
        .map(|_aabb: (AABB, usize)| (_aabb.1.to_owned(), Box::new(_aabb.0) as Box<dyn Bounding>))
        .chain(
            obb.to_owned().into_iter().map(|_obb: (OBB, usize)| {
                (_obb.1.to_owned(), Box::new(_obb.0) as Box<dyn Bounding>)
            }),
        )
        .chain(
            sphere
                .to_owned()
                .into_iter()
                .map(|_sphere: (Sphere, usize)| {
                    (
                        _sphere.1.to_owned(),
                        Box::new(_sphere.0) as Box<dyn Bounding>,
                    )
                }),
        )
        .collect();

    let mut bounding_intersect: Vec<(usize, usize)> = vec![];
    boundings.into_iter().for_each(|(id, bounding)| {
        aabb.to_owned().into_iter().for_each(|(other, other_id)| {
            if id != other_id && bounding.intersects_aabb(&other)  {
                bounding_intersect.push((id, other_id));
            }
        });
        obb.to_owned().into_iter().for_each(|(other, other_id)| {
            if id != other_id && bounding.intersects_obb(&other)  {
                bounding_intersect.push((id, other_id));
            }
        });
        sphere.to_owned().into_iter().for_each(|(other, other_id)| {
            if id != other_id && bounding.intersects_sphere(&other) {
                bounding_intersect.push((id, other_id));
            }
        });
    });

    HttpResponse::Ok().json(bounding_intersect)
}

///  End points de visualização
///
///
// Endpoint para visualização da soma
#[get("/soma")]
async fn view_sum() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PathBuf::from(
        "./static/html/sum-visualization.html",
    ))?)
}

// Endpoint para visualização da reação
#[get("/reacao")]
async fn view_reaction() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PathBuf::from(
        "./static/html/reaction-visualization.html",
    ))?)
}

// Endpoint para visualização da intersecção de segmentos de reta
#[get("/interseccao")]
async fn view_intersection() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PathBuf::from(
        "./static/html/intersection-visualization.html",
    ))?)
}

// Endpoint para visualização da reação
#[get("/colisao")]
async fn view_colision() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PathBuf::from(
        "./static/html/colision-visualization.html",
    ))?)
}

// Endpoint para visualização de ângulos
#[get("/angulos")]
async fn view_angles() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PathBuf::from(
        "./static/html/angle-visualization.html",
    ))?)
}

// Endpoint para visualização de contrução de volumes envoltórios
#[get("/envoltorios-construtor")]
async fn view_envelopes_contructor() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PathBuf::from(
        "./static/html/envelope-constructor-visualization.html",
    ))?)
}

// Configuração das rotas
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(soma_vetores)
            .service(subtracao_vetores)
            .service(redimensionar)
            .service(produto_escalar)
            .service(produto_vetorial)
            .service(projecao_vetores)
            .service(reacao_vetores)
            .service(normal_segmento)
            .service(intersecsao_segmento)
            .service(segmentos_intersectam)
            .service(colisao)
            .service(angulos)
            .service(colisao_refletida)
            .service(envelopes_contructor)
            .service(envelopess_intersectam)
            .service(decomposicao_vetores),
    )
    .service(view_sum)
    .service(view_reaction)
    .service(view_intersection)
    .service(view_colision)
    .service(view_angles)
    .service(view_envelopes_contructor)
    .service(app_home);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Abertura da página no navegador
    println!("Starting server at http://127.0.0.1:8080");
    if let Err(e) = webbrowser::open("http://127.0.0.1:8080/envoltorios-construtor") {
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
            .service(fs::Files::new("/static", "./static"))
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

#[derive(Deserialize)]
struct FindIntersectingRequest {
    segments: Vec<(Vector, Vector)>,
}

#[derive(Deserialize)]
struct FindAnglesRequest {
    vectors: Vec<Vector>,
}

#[derive(Deserialize)]
struct ReflectedColisionRequest {
    walls: Vec<(Vector, Vector)>,
    vector: (Vector, Vector),
}

#[derive(Deserialize)]
struct BoundingBoxRequest {
    points: Vec<Vector>,
    bounding_type: String,
}

#[derive(Deserialize)]
struct BoundingBoxCollisionRequest {
    aabb: Vec<(AABB, usize)>,
    obb: Vec<(OBB, usize)>,
    sphere: Vec<(Sphere, usize)>,
}
