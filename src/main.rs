use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::rt::{time, spawn};

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
use std::{convert::Infallible, time::Duration};

#[get("/from-channel")]
async fn from_channel() -> impl Responder {
    println!("from_channel");
    let (sender, sse_stream) = sse::channel(10);


    spawn(async move {
        for i in 1..=10 {
            let _ = sender.send(sse::Data::new(i.to_string())).await;
            sender.send(sse::Event::Comment("my comment".into())).await;
            time::sleep(Duration::from_secs(1)).await;
        }
    });
    // note: sender will typically be spawned or handed off somewhere else
    // let _ = sender.send(sse::Event::Comment("my comment".into())).await;
    // let _ = sender
    //     .send(sse::Data::new("my data").event("chat_msg"))
    //     .await;

    sse_stream.with_retry_duration(Duration::from_secs(5))
}

#[get("/from-stream")]
async fn from_stream() -> impl Responder {
    let event_stream =
        futures_util::stream::iter([Ok::<_, Infallible>(sse::Event::Data(sse::Data::new("foo")))]);

    sse::Sse::from_stream(event_stream).with_keep_alive(Duration::from_secs(5))
}
