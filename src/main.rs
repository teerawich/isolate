use actix_web::{ HttpServer, web, App};

mod application;
mod branch;
mod project;
mod ticket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("My Application version: {}", VERSION);

    HttpServer::new(||{
        App::new()
          .service(
            web::scope("/projects")
              .service(project::project_list)
              .service(project::create_project)
              .service(project::remove_project)

          )
          .service(
            web::scope("/branches")
              .service(branch::branch_list)
              .service(branch::create_branch)
              .service(branch::remove_branch)
          )
          .service(
            web::scope("/tickets")
              .service(ticket::ticket_list)
              .service(ticket::create_ticket())
              .service(ticket::remove_ticket())
          )
          .service(application::ping)
          .service(application::ping_id)
    })
      .bind("127.0.0.1:8080")?
      .run()
      .await
}
