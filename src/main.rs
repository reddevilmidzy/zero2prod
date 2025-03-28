use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // conf에서 값 가져와서 포트 바인딩하기
    let configuration = get_configurations().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let addr = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(addr)?;
    run(listener, connection_pool)?.await
}
