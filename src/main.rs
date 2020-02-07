use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hi Rusty Zombieland")
}

async fn trash() -> impl Responder {
    HttpResponse::Ok().body("Hello World Basura!")
}

#[get("/macro")]
async fn services() -> impl Responder {
    HttpResponse::Ok().body("Ok, then we get a Macro Response")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(trash))
            .service(services)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}