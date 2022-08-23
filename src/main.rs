use std::{env, net::SocketAddr, sync::Arc, time::Duration};

use axum::{routing::get, AddExtensionLayer, Router};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

mod entity;
mod handler;

pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let mut opt = ConnectOptions::new(env::var("DB_HOST").expect("DB_HOST is not defined"));
    opt.max_connections(10000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);
    let db: DatabaseConnection = Database::connect(opt)
        .await
        .expect("Database connection failed");

    let app_state = Arc::new(AppState { db });

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT")
            .expect("PORT is not defined")
            .parse::<u16>()
            .unwrap(),
    ));

    if let Err(e) = axum::Server::bind(&addr)
        .serve(app(app_state).into_make_service())
        .await
    {
        panic!("{:?}", e)
    }
}

fn app(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/todo",
            get(handler::todo::get_todo_list).post(handler::todo::post_todo),
        )
        .layer(AddExtensionLayer::new(app_state))
}
