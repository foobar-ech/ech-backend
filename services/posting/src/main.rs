use actix_web::{App, HttpServer, Responder, Result, error::ErrorInternalServerError, get, web};
use common::SharedConfig;
use serde::Deserialize;
use sqlx::{
    MySqlPool, Row,
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
};

pub(crate) mod dto;
pub(crate) mod row;

use crate::{dto::board::BoardDto, row::board::BoardRow};

#[derive(Deserialize)]
struct Config {
    db_user: String,
    db_pass: String,
    db_name: String,
}

#[get("/boards")]
async fn boards(pool: web::Data<MySqlPool>) -> Result<impl Responder> {
    let rows = sqlx::query_as::<_, BoardRow>(
        r#"
        SELECT
            boards.*,
            CAST(boards.ban_reasons as CHAR) AS ban_reasons,
            CAST(boards.spamlist as CHAR) AS spamlist,
            CAST(boards.replacements as CHAR) AS replacements
        FROM boards"#,
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;
    let ctx = sqlx::query("SELECT board, count(id) FROM posts GROUP BY board")
        .fetch_all(pool.get_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|r| (r.get(0), r.get(1)))
        .collect();

    let dtos: Vec<BoardDto> = rows
        .into_iter()
        .map(|r| BoardDto::try_from((r, &ctx)))
        .collect::<Result<_, _>>()
        .map_err(ErrorInternalServerError)?;

    Ok(web::Json(dtos))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    _ = dotenvy::from_path("config/shared.env");
    _ = dotenvy::from_path("config/posting.env");

    let shared_config = envy::from_env::<SharedConfig>().unwrap();
    let config = envy::from_env::<Config>().unwrap();

    let options = MySqlConnectOptions::new()
        .host(&shared_config.db_host)
        .port(shared_config.db_port)
        .username(&config.db_user)
        .password(&config.db_pass)
        .database(&config.db_name);

    let pool = MySqlPoolOptions::new().connect_with(options).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(boards)
    })
    .bind(("localhost", shared_config.port_posting))?
    .run()
    .await
}
