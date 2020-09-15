use actix_web::{HttpResponse, web};
use actix_web_codegen::get;
use actix_web::web::Json;
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Client {
  user_id: String,
}

#[get("/ping")]
pub async fn ping() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("pong!"))
}

#[get("/ping/{id}")]
pub async fn ping_id(web::Path(id) : web::Path<String>) -> Result<Json<Client>, actix_web::Error> {
  Ok(Json(Client{user_id: id}))
}