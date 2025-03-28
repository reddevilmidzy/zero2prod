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


### SQLx

sqlx는 비동기 인터페이스를 가짐. 그러나 같은 db 커넥션에 대해 동시에 여러 쿼리를 실행할 수는 없다. 가변 참조자(mutable reference)를 요청하면 API에서 이를 보장하도록 강제한다.  
가변 참조자는 고유한 참조(unique reference)처럼 생각할 수 있다. 컴파일러는 그들이 실제로 그 PgConnection에 배타적 접근할 수 있음을 보장한다. 전체 프로그램에서 같은 시간에 같은 값에 대한 두 개의 활성화된 가변 참조자는 존재할 수 없기 때문이다.  
PgConnection을 lock(Mutex) 뒤에 놓음으로써 기반 TCP 소켓에 대한 접속을 동기화화고, lock을 얻은 뒤 감싸인 커넥션에 대한 가변 참조자를 얻어 동작을 하게 할 수 있다. 하지만 이상적이지는 않음. 한 순간에 하나의 쿼리만 실행할 수 밖에없기 때문.  
PgPool을 사용하면 해결 가능하다.  

### 도서 글귀

* 영속성 요규가 명확하지 않다면, 관게형 데이터베이스를 사용하자. 큰 확장을 예상할 필요가 없다면, PostgreSQL을 사용하자.
* 스레드는 병렬로 동작한다. 비동기는 병렬로 대기한다. 

### 학습 키워드 

* Arc: connection을 Arc에 담아서 넘겼다. 


[깃허브 저장소](https://github.com/LukeMathWalker/zero-to-production)
