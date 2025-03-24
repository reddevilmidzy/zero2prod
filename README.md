# zero2prod
제로부터 시작하는 러스트 백엔드 프로그래밍

### 사용자 스토리
블로그 방문자로서,  
뉴스레터를 구독하기를 원한다.  
그래야 새로운 컨텐츠가 블로그에 게시되었을 때 이메일로 알림을 받을 수 있다.  


### Serde

* serde는 러스트 데이터 구조를 효율적이고 제너릭하게 직렬화/역직렬화하기 위한 프레임워크이다.

```rust
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
```

* subscribe를 호출하기 전에 actix-web은 from_request 메서드를 모든 subscribe의 입력 인자에 대해 호출
* 바디를 역직렬화하고 URL 인코딩 규칙에 따라 FormData로 만듦. 이때 serde_urlencoded와 FormData의 Deserialize 구현을 활용. (#[derive(serde:Deserialize)]에 의해 자동 생성)
* from_request가 실패하면, 400 BAD Request가 호출자에게 반환된다. 성공하면 subscribe가 호출되고 200 Ok를 반환.

[깃허브 저장소](https://github.com/LukeMathWalker/zero-to-production)
