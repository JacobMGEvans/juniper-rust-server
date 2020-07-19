extern crate juniper;
use std::io;


use actix_web::{web, App, HttpResponse, HttpServer, Responder};




mod gql_schema;
use crate::gql_schema::{create_schema, Schema};

fn main() -> io::Result<()> {
    // HttpServer::new(|| {
    //     App::new()
    //         .route("/", web::get().to(index))
    // })
    // .bind("localhost:4321")?
    // .run()
    let schema = std::sync::Arc::new(create_schema());
        HttpServer::new(move || {
            App::new()
                .data(schema.clone())
                .service(web::resource("/graphql").route(web::post().to_async(graphql)))
                .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        })
        .bind("localhost:8080")?
        .run()
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
