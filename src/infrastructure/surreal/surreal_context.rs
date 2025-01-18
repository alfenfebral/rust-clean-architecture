use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};
use std::env;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_db() -> Result<()> {
    let db_url = env::var("DB_URL").expect("DB_URL must be set");
    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

    let _ = DB.connect::<Ws>(db_url).await?;
    let _ = DB
        .signin(Root {
            username: &db_username,
            password: &db_password,
        })
        .await;
    let _ = DB.use_ns("todo").use_db("todo").await?;
    Ok(())
}