// from https://oneuptime.com/blog/post/2026-02-01-rust-actix-web-rest-api/view
// stripped down to force server to handle POST requests
// needs to be identified in [package] section of cargo.toml:  default-run = "rust-api" 
use actix_web::{App, HttpServer, post, get, Responder, HttpResponse};
//use actix_web_helmet::{Helmet, HelmetMiddleware, ContentSecurityPolicy};
use std::io::{Read, Write, LineWriter};
use std::fs::OpenOptions;
use chrono::prelude::*;
use html_escape::encode_text;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("Starting server on 0.0.0.0:8080");
    // let helmet: HelmetMiddleware = Helmet::default().try_into().expect("valid headers");
    //let csp = ContentSecurityPolicy::default()
    //    .style_src(vec!["'self'"]).font_src(vec!["'self'"]).connect_src(vec!["'self'"]);
    //let helmet: Helmet = Helmet::default().add(csp);
    //let mid_helmet: HelmetMiddleware = helmet.try_into().expect("valid headers");
    HttpServer::new(move || {
        App::new()
            //.wrap(mid_helmet.clone())
            .service(submit_violation)
            .service(violations)
            .service(index)
        })
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
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("violations.log")
        .expect("Failed to open file");
    let mut outfile = LineWriter::new(file);
    let tz_body = format!("{},\"event-timestamp\":\"{}\"}}}}\n",&short_body[..short_body.len()-2], utc.to_rfc3339_opts(SecondsFormat::Secs, true)).into_bytes();
    //log::info!("Body with timezone: {}", tz_body);
    let _ = outfile.write_all(&tz_body);
    //writeln!(file, "{}", tz_body).expect("Write failure");
    let _ = outfile.flush(); //make sure the newline is written after each report line is written
    //log::info!("Saved string len: {}", short_body.len());
    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<endpoint>")
}

//abui3o  m35qrk  jrusp6
#[get("/abui3o/m35qrk")]
async fn violations() -> impl Responder {
    log::info!("Fetching violations");
    let mut outmessage = String::new();
    let file = OpenOptions::new()
    .write(false)
    .read(true)
    .open("violations.log");
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("Failed to read file");
            outmessage.push_str(encode_text(&contents).as_ref());
        },
        Err(e) => {
            log::error!("Error opening file: {}", e);
            outmessage.push_str("Error reading violations");
        }
    }
   
    HttpResponse::Ok().body(outmessage)
}