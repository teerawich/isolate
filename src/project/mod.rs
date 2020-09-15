use actix_web::{HttpResponse};
use actix_web_codegen::{get, post, delete};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Project {
  id: u32,
  name: String,
}

#[get("/")]
pub async fn project_list() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("project list"))
}

#[post("/")]
pub async fn create_project() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("new project"))
}

#[delete("/{id}")]
pub async fn remove_project() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("remove project"))
}