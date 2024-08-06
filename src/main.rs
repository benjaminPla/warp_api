use crate::helpers::{create_pool, setup_database};
use crate::routes::create_routes;
use dotenv::dotenv;
use std::{env, net::SocketAddr};

mod controllers;
mod errors;
mod helpers;
mod middlewares;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = create_pool().await.expect("Failed to create pool");
    setup_database(pool.clone())
        .await
        .expect("Failed setting up the database");

    let port = env::var("PORT")
        .expect("Missing \"PORT\" env variable")
        .parse::<u16>()
        .expect("PORT must be a number");
    let routes = create_routes(pool);
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();

    warp::serve(routes).run(addr).await;
}
