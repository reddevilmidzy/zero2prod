use tokio::task::JoinHandle;
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    // https://doc.rust-lang.org/nomicon/hrtb.html를 참고해보자.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // RUST_LOG 환경 변수가 설정되어 있지 않으면 info 및 그 이상의 레벨의 모든 span을 출력
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // 포맷이 적용된 span들을 stdout으로 출력
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // `with` 메서드는 `SubscriberExt`에서 제공. 이 녀석은 `Subscriber`의 확장 트레이트며, `tracing_subscriber`에 의해 노출됨
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // 모든 `log`의 이벤트를 구독자에게 리다이렉트
    LogTracer::init().expect("Failed to set logger");
    // 어떤 subscriber를 사용해야 하는지 지정할 수 있음
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    // 이것이 실행된 뒤 새로운 스레드를 실행
    let current_span = tracing::Span::current();
    // 그 뒤 스레드의 소유권을 클로저에 전달하고
    // 그 스코프 안에서 명시적으로 모든 계산을 실행,
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
