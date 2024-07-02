use std::iter::Map;

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
    result: ResultInfo,
    code: i16,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct ResultInfo{
    user_id: String,
    refresh_token: String,
    access_token: String,
}



#[post("/v2/lr/login_phone")]
async fn login_phone() -> Result<impl Responder> {
    let ri = ResultInfo{
        user_id: String::from("1234"),
        refresh_token: String::from("12345"),
        access_token: String::from("12345"),
    };
    let obj = MyObj {
        result: ri,
        code: 200,
        message: String::from("test")
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
            .service(login_phone)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8095))?
    .run()
    .await
}
