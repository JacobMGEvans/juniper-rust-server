extern crate juniper;

use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;


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

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,) -> impl Future<Item = HttpResponse , Error = Error>{
        web::block(move || {
            let res = data.execute(&st, &());
            Ok::<_,serde_json::error::Error>(serde_json::to_string(&res)?)
            .map_err(Error::from)
            .and_then(|user| {
                Ok(HttpResponse::OK()
            .content_type("application/json")
            .body(user))
            })
        })
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:4321/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
