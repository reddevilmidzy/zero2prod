use std::net::TcpListener;

#[tokio::test]
async fn health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=redddy&email=hello@redddy.com";
    let response = client
        .post(&format!("{}/subscriptions", &app))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=redddy", "missing the email"),
        ("email=hello@redddy.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind random port");

    // OS가 할당한 포트 추출
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("Failed to bind address");
    // 서버를 백그라운드로 구동
    // torio::spawn은 생성된 퓨처에 대한 핸들을 반환한다.
    // 하지만 여기에서는 사용하지 않으므로 바인딩하진 않음
    let _ = tokio::spawn(server);
    //애플리케이션 주소를 호출자에게 반환
    format!("http://localhost:{}", port)
}
