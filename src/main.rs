extern crate mpd;

use actix_rt;
use actix_cors::Cors;
use actix_web::{post, App, HttpRequest, HttpServer, web, HttpResponse};
use mpd::{Client, Song};
use serde::{Deserialize, Serialize};
use crate::schedule::schedule::{CronSchedule, ScheduleStart};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

mod schedule;

#[derive(Deserialize,Serialize)]
struct PlaySong {
    url: String
}

lazy_static!{
    static ref SCHED: Mutex<CronSchedule> = Mutex::new(CronSchedule::new());
}

#[post("/play")]
async fn play(item: web::Json<PlaySong>) -> HttpResponse {
    let song = Song {
        file: item.url.clone(),
        name: None,
        title: None,
        last_mod: None,
        duration: None,
        place: None,
        range: None,
        tags: Default::default()
    };
    let mut conn = Client::connect("localhost:6600").unwrap();
    conn.clear().unwrap();
    conn.push(song).unwrap();
    conn.play().unwrap();
    HttpResponse::Ok().body("OK")
}

#[post("/stop")]
async fn stop(_req: HttpRequest) -> HttpResponse {
    let mut conn = Client::connect("localhost:6600").unwrap();
    conn.stop().unwrap();
    conn.clear().unwrap();
    HttpResponse::Ok().body("OK")
}

#[post("/startAt")]
async fn schedule_on(item: web::Json<ScheduleStart>) -> HttpResponse {
    SCHED.lock().unwrap().add_schedule_start(&item.into_inner());
    HttpResponse::Created().body("Created")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    print!("Starting on port 8080");
    let server = HttpServer::new(|| {
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
            .service(schedule_on)
            .service(stop)
            .service(play)
    })
        .bind("0.0.0.0:8080")?
        .run();

    loop {
        SCHED.lock().unwrap().tick();
        sleep(Duration::from_millis(1000));
    }
}
