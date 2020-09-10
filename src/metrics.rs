use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::debug;
use prometheus::{Counter, Encoder, TextEncoder};

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "http_requests_total",
        "Total number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
}

async fn metrics(_req: HttpRequest) -> impl Responder {
    HTTP_COUNTER.inc();

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let buffer_string = String::from_utf8(buffer).unwrap();

    HttpResponse::Ok().body(&buffer_string)
}

pub async fn server() -> std::io::Result<()> {
    debug!("Metrics server starting.");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/health").to(|| async { "Alive!" }))
            .service(web::resource("/metrics").to(metrics))
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
