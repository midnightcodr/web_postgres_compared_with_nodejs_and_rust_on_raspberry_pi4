
mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Config {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "posts")] 
    pub struct Post {
        pub id: i32,
        pub title: String,
        pub body: String,
        pub published: bool,
    }
}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod db {
    use crate::{errors::MyError, models::Post};
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    pub async fn get_posts(client: &Client) -> Result<Vec<Post>, MyError> {
        let _stmt = String::from("select id, title, body, published from posts where published='t'");
        let stmt = client.prepare(&_stmt).await.unwrap();

        let rows = client
            .query(
                &stmt, &[]
            )
            .await?;
        Ok(rows
            .iter()
            .map(|row| Post::from_row_ref(row).unwrap())
            .collect::<Vec<Post>>()
        )
    }
}

mod handlers {
    use crate::{db, errors::MyError};
    use actix_web::{web, Error, HttpRequest, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    pub async fn get_posts(
        _: HttpRequest,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
        let posts = db::get_posts(&client).await?;
        Ok(HttpResponse::Ok().json(posts))
    }
}

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::get_posts;
use tokio_postgres::NoTls;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/posts").route(web::get().to(get_posts)))
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
