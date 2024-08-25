use std::io;
use ntex::web::{App, HttpResponse, HttpServer};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use tracing::{error, info, instrument, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tracing_subscriber::util::SubscriberInitExt;

#[instrument]
#[ntex::web::get("/")]
pub async fn index() -> HttpResponse {
    info!("Call index");
    HttpResponse::Ok().body("Hello")
}

#[ntex::main]
async fn main() -> io::Result<()>{
    let provider = TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("readme_example");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    subscriber.init();
    // let _guard = tracing::subscriber::set_default(subscriber);
    {
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();
        error!("This event will be logged in the root span.");
    }

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
