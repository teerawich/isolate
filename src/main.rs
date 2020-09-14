use actix_web::{ HttpServer, App};

mod application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("My Application version: {}", VERSION);

    HttpServer::new(||{
        App::new()
          .service(application::ping)
          .service(application::ping_id)
    })
      .bind("127.0.0.1:8080")?
      .run()
      .await
}
