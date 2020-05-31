use actix_web::{App, HttpServer};
use std::{io, env};
use crate::config::*;
use actix_cors::Cors;
use crate::middleware;

pub struct Router {
}
impl Router {
    pub fn new() -> Self {
        Router {
        }
    }

    pub async fn load(&self) -> io::Result<()> { 
        let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
        let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
        let app_url = format!("{}:{}", &app_host, &app_port);
        
        HttpServer::new(move || 
            App::new()
            .wrap(Cors::new() // allowed_origin return access-control-allow-origin: * by default
            // .allowed_origin("http://127.0.0.1:3000")
            .supports_credentials()
            // .send_wildcard()
            // .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // .allowed_header(http::header::CONTENT_TYPE)
            // .max_age(3600)
            .finish())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::authen::Authentication)
            // .wrap(middleware::read_response_body::Logging)
            .configure(app::config_services)
        )
        .bind(&app_url)?
        .run()
        .await
    }
}