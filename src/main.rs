use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // init을 하여 set_logger 설정.
    // RUST_LOG 환경 변수가 설정되어 있지 않으면 info 및 그 이상의 레벨의 모든 로그를 출력
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    // conf에서 값 가져와서 포트 바인딩하기
    let configuration = get_configurations().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let addr = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(addr)?;
    run(listener, connection_pool)?.await
}
