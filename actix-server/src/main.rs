use actix_web::{App, HttpResponse, HttpServer, Responder, web, middleware::Logger};
use env_logger::Env;

#[actix_web::get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Moin Welt")
}

#[actix_web::get("/echo/{text}")]
async fn echo(text: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(text.to_string())
}

#[actix_web::get("/livz")]
async fn livz() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::get("/healthy")]
async fn healthy() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(echo)
            .service(livz)
            .service(healthy)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
