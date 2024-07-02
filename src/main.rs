/*
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/login_phone")]
async fn login_phone(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/signup")]
async fn signup(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/issign")]
async fn issign() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[post("/hot_act")]
async fn hot_act(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

#[get("/a/{name}")]
async fn index(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(login_phone).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
} 
*/
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

#[post("/v2/lr/login_phone")]
async fn index(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyObj {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8095))?
    .run()
    .await
}
