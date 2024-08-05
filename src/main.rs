use crate::database::create_pool;
use crate::database::setup_database;
use crate::routes::create_routes;
use dotenv::dotenv;
use std::net::SocketAddr;

mod controllers;
mod database;
mod middlewares;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = create_pool().await.expect("Failed to create pool");
    setup_database(pool.clone())
        .await
        .expect("Failed setting up the database");

    let routes = create_routes(pool);
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    warp::serve(routes).run(addr).await;
}
