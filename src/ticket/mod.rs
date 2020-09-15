use actix_web::{HttpResponse};
use actix_web_codegen::{get, post, delete};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Ticket {
  id: u32,
  name: String,
}

#[get("/")]
pub async fn ticket_list() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("ticket list"))
}

#[post("/")]
pub async fn create_ticket() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("new ticket"))
}

#[delete("/{id}")]
pub async fn remove_ticket() -> Result<HttpResponse, actix_web::Error> {
  Ok(HttpResponse::Ok().body("remove"))
}