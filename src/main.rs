extern crate mpd;

use actix_rt;
use actix_cors::Cors;
use actix_web::{App, HttpRequest, HttpServer, Responder, web, HttpResponse};
use mpd::{Client, Song};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
struct PlaySong {
    url: String
}

async fn play(item: web::Json<PlaySong>) -> HttpResponse {
    let song = Song { file: item.url.clone(), name: None, title: None, last_mod: None, duration: None, place: None, range: None, tags: Default::default() };
    let mut conn = Client::connect("pi-radio.localdomain:6600").unwrap();
    conn.clear().unwrap();
    conn.push(song).unwrap();
    conn.play().unwrap();
    HttpResponse::Ok().body("OK")
}

async fn stop(_req: HttpRequest) -> impl Responder {
    let mut conn = Client::connect("pi-radio.localdomain:6600").unwrap();
    conn.stop().unwrap();
    conn.clear().unwrap();
    return format!("OK");
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    print!("Starting on port 8080");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("http://localhost")
                    .allowed_origin("http://localhost:4200")
                    .allowed_origin("http://pi-radio.localdomain")
                    .allowed_origin("http://pi-radio")
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600)
                    .finish())
            .route("/play", web::post().to(play))
            .route("/stop", web::post().to(stop))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
