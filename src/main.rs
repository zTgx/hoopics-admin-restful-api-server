// #[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

use std::env;

mod config;
mod constants;
mod controller;
mod middleware;
mod models;
mod service;
mod util;
mod hi;
mod router;
mod schema;

use crate::router::Router;
use crate::hi::rustaceans;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // util::default_log::init_log();
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info,debug");
    env_logger::init();

    rustaceans();

    //加载Router
    Router::new().load().await
}
   