use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::configuration::get_configurations;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 모든 `log`의 이벤트를 구독자에게 리다이렉트
    LogTracer::init().expect("Failed to set logger");
    // RUST_LOG 환경 변수가 설정되어 있지 않으면 info 및 그 이상의 레벨의 모든 span을 출력
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 포맷이 적용된 span들을 stdout으로 출력
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);

    // `with` 메서드는 `SubscriberExt`에서 제공. 이 녀석은 `Subscriber`의 확장 트레이트며, `tracing_subscriber`에 의해 노출됨
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // 어떤 subscriber를 사용해야 하는지 지정할 수 있음
    set_global_default(subscriber).expect("Failed to set subscriber");

    // conf에서 값 가져와서 포트 바인딩하기
    let configuration = get_configurations().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let addr = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(addr)?;
    run(listener, connection_pool)?.await
}
