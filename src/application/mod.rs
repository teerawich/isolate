use actix_web::{HttpResponse, web};
use actix_web_codegen::get;

#[get("/ping")]
pub async fn ping() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("pong!"))
}

#[get("/ping/{id}")]
pub async fn ping_id(web::Path(id) : web::Path<u32>) -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body(format!("Hello {}!", id)))
}