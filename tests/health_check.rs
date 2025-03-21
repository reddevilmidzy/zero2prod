#[tokio::test]
async fn health_check() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to start zero2prod");
    // 서버를 백그라운드로 구동
    // torio::spawn은 생성된 퓨처에 대한 핸들을 반환한다.
    // 하지만 여기에서는 사용하지 않으므로 바인딩하진 않음
    let _ = tokio::spawn(server);
}
