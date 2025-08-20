use crate::cache::Cache;
use crate::job::run;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::{State, get, routes};
use rocket_cors::{AllowedOrigins, CorsOption};
use std::sync::Mutex;

mod cache;
mod job;
mod model;

#[get("/news")]
async fn news_handler(cache: &State<Mutex<Cache>>) -> Result<String, Status> {
    let mut store = cache.lock().unwrap();

    match store.get_news() {
        None => Err(Status::NotFound),
        Some(res) => Ok(res),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    tokio::spawn(async {
        run().await;
    });
    let cache = Mutex::new(Cache::new());
    let cors = CorsOptions::default().allowed_origins(AllowedOrigins::all());

    rocket::build()
        .manage(cache)
        .mount("/", routes![news_handler])
        .attach(cors.to_cors().unwrap())
        .attach(AdHoc::config::<rocket::Config>())
        .configure(rocket::Config {
            port: 4000,
            ..rocket::Config::default()
        })
        .launch()
        .await
        .unwrap();

    Ok(())
}
