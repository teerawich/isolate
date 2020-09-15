use actix_web::{HttpResponse};
use actix_web_codegen::{get, post, delete};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Branch {
  id: u32,
  name: String,
}

#[get("/")]
pub async fn branch_list() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("branch list"))
}

#[post("/")]
pub async fn create_branch() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("new branch"))
}

#[delete("/{id}")]
pub async fn remove_branch() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("remove"))
}