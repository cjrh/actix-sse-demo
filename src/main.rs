use actix_web::rt::{spawn, time};
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

#[get("/")]
async fn hello() -> impl Responder {
    let html_file = std::fs::read_to_string("index.html").unwrap();
    HttpResponse::Ok().body(html_file)
}

#[post("/hello")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("hello")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .service(from_channel)
            .service(from_stream)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

use actix_web_lab::sse;
use std::thread::sleep;
use std::{convert::Infallible, time::Duration};
use tokio::task::spawn_blocking;

#[get("/from-channel")]
async fn from_channel() -> impl Responder {
    println!("from_channel");

    // my producer
    let (sender, receiver) = mpsc::channel(10);
    spawn(async move {
        for i in 1..=10 {
            let _ = sender.send(i.to_string()).await;
            time::sleep(Duration::from_secs(1)).await;
        }
    });

    sse::Sse::from_stream(
        ReceiverStream::new(receiver)
            .map(|string| Ok::<_, Infallible>(sse::Event::Data(sse::Data::new(string)))),
    )
}

#[get("/from-stream")]
async fn from_stream() -> impl Responder {
    let event_stream =
        futures_util::stream::iter([Ok::<_, Infallible>(sse::Event::Data(sse::Data::new("foo")))]);

    sse::Sse::from_stream(event_stream).with_keep_alive(Duration::from_secs(5))
}
