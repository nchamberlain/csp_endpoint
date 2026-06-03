// from https://oneuptime.com/blog/post/2026-02-01-rust-actix-web-rest-api/view
// stripped down to force server to handle POST requests
// needs to be identified in [package] section of cargo.toml:  default-run = "rust-api" 
use actix_web::{App, HttpServer, post, get, Responder, HttpResponse};
use std::io::Write;
use std::fs::OpenOptions;
use chrono::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("Starting server on 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .service(submit_violation)
            .service(index)
    })//.workers(8) //  didn't help reduce D.O. consoles - App was set to have 2 containers
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[post("/fnirc7")]
pub async fn submit_violation(body: String) -> std::io::Result<()> {
    // truncate excessive length body - probably hacking trying for buffer overflow.
    // If results prove otherwise, the lengths can be adusted
    let utc: DateTime<Utc> = Utc::now();
    let short_body: &str;
    if body.len() > 2000 { //body.len is in bytes. Short_body is in chars to handle full UTF-8
        let mut indices = body.char_indices();
        let start = indices.nth(0).map(|(i,_)| i).unwrap_or(body.len());
        let end = indices.nth(1000).map(|(i, _)| i).unwrap_or(body.len());
        short_body = &body[start..end];
    } else {
        short_body = &body[..];
    }
    log::info!("body: {:?} [[length: {}]]", short_body, short_body.len());
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("violations.log")
    .expect("Failed to open file");
    let tz_body = format!("[{}] {}\n", utc.to_rfc3339_opts(SecondsFormat::Secs, true), short_body);
    //log::info!("Body with timezone: {}", tz_body);
    writeln!(file, "{}", tz_body).expect("Write failure");
    //log::info!("Saved string len: {}", short_body.len());
    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<endpoint>")
}
