# zero2prod
제로부터 시작하는 러스트 백엔드 프로그래밍

### 사용자 스토리
블로그 방문자로서,  
뉴스레터를 구독하기를 원한다.  
그래야 새로운 컨텐츠가 블로그에 게시되었을 때 이메일로 알림을 받을 수 있다.  


블로그 저자로서,
나는 모든 확인된 구독자에게 이메일을 보내기를 원한다.
그래야 새로운 콘텐츠를 블로그에 게시했을 때 구독자들에게 알릴 수 있다.


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

<br>

버전 0.8에는 offline이 없는건가?  


### 로깅

tracing이 갓갓이다.  
<br>
[참고](https://docs.rs/tracing/latest/tracing/trait.Subscriber.html)

포맷한 로그  

```text
{"v":0,"name":"zero2prod","msg":"[ADDING A NEW SUBSCRIBER. - START]","level":30,"hostname":"DESKTOP-WEONTOP","pid":23468,"time":"2025-03-28T10:48:17.5413366Z","target":"zero2prod::routes::subscriptions","line":14,"file":"src\\routes\\subscriptions.rs","subscriber_name":"\"hee\"","subscriber_email":"\"tmp4@gmai.com\"","request_id":"334e64ef-5378-4ef3-b70d-8e763681088e"}
{"v":0,"name":"zero2prod","msg":"[SAVING NEW SUBSCRIBER DETAILS IN THE DATABASE - START]","level":30,"hostname":"DESKTOP-WEONTOP","pid":23468,"time":"2025-03-28T10:48:17.5422252Z","target":"zero2prod::routes::subscriptions","line":24,"file":"src\\routes\\subscriptions.rs","subscriber_name":"\"hee\"","subscriber_email":"\"tmp4@gmai.com\"","request_id":"334e64ef-5378-4ef3-b70d-8e763681088e"}
{"v":0,"name":"zero2prod","msg":"[SAVING NEW SUBSCRIBER DETAILS IN THE DATABASE - END]","level":30,"hostname":"DESKTOP-WEONTOP","pid":23468,"time":"2025-03-28T10:48:17.5494093Z","target":"zero2prod::routes::subscriptions","line":24,"file":"src\\routes\\subscriptions.rs","subscriber_name":"\"hee\"","subscriber_email":"\"tmp4@gmai.com\"","request_id":"334e64ef-5378-4ef3-b70d-8e763681088e","elapsed_milliseconds":0}
{"v":0,"name":"zero2prod","msg":"[ADDING A NEW SUBSCRIBER. - END]","level":30,"hostname":"DESKTOP-WEONTOP","pid":23468,"time":"2025-03-28T10:48:17.5498357Z","target":"zero2prod::routes::subscriptions","line":14,"file":"src\\routes\\subscriptions.rs","subscriber_name":"\"hee\"","subscriber_email":"\"tmp4@gmai.com\"","request_id":"334e64ef-5378-4ef3-b70d-8e763681088e","elapsed_milliseconds":7}
```

<br>

tracing-actix-web은 actix-web의 Logger를 대체하기 위해 설계되어 있으며, log가 아닌 tracing에 기반을 둠. 매 요청마다 일일이 작성해줄 필요가 없다.  
또한 tracing-opentelemetry를 설치하면 span을 OpenTelemetry 호환 서비스([Jagger](https://www.jaegertracing.io), [Honeycomb.io](https://honeycomb.io) 등)로 보내 심층적 분석 가능

### 도커

#### sqlx 오프라인 모드
바로 도커 빌드하면 오류가 발생한다. 이유는 sqlx 때문인데, sqlx는 테이블의 스키마를 고려해 모든 쿼리가 성공적으로 실행될 수 있도록 컴파일 시 데이터베이스를 호출한다. 하지만 cargo build를 도커 이미지 안에서 실행하면, sqlx는 .env 파일의 DATABASE_URL 환경 변수가 가리키는 데이터베이스와 커넥션을 만드는데 실패한다.  

```shell
cargo sqlx prepare -- --lib
```

도커 컨테이너 빌드 명령  

```shell
docker build --tag zero2prod --file Dockerfile .
```

도커 실행 명령  
```shell
docker run zero2prod
```

포트 노출하면서 실행  
이게 필요한 이유는 기본적으로 도커 이미지는 기반 호스트 머신에 포트를 노출하지 않음
```shell
docker run -p 8080:8080 zero2prod
```

```shell
curl --request POST --data 'name=redddy&email=hello@redddy.com' 127.0.0.1:8080/subscriptions --verboes
```
로 요청을 보내면 약 30초 정도가 지난 후 500이 돌아온다.  

이유는 connect를 connect_lazy로 바꿔 db를 직접 다루는 것을 회피했기 때문이다.  

### 도커 이미지 최적화하기

도커 파일에서 최적화할 수 있는 것은 **더 빠른 사용을 위한 작은 이미지 크기**와 **더 빠른 빌드를 위한 도커 레이어 캐싱**이 있다.  

러스트에서 docker build는 꽤 오래 걸린다. 


**step1** 
<br>

최적화하기 전에는 7.32GB 크기의 이미지였다.  

![최적화 전 이미지 사이즈](img/img.png)

[최적화 전 Dockerfile 코드](dockerfile_history/start)  
<br>

**step2**
<br>

다단계 빌드를 사용하고 이미지를 빌드하는데 필요하지 않은 파일을 제거하기 위해 .dockerignore 파일에 명시를 해주고, 다단계 빌드를 사용하여 최적화 하니 820MB로 줄었다. 
이때 rust 버전에 -slim이 붙은 친구를 사용했는데 동일한 기반 OS를 사용하는 더 작은 이미지다. 러스트 툴체인과 용병(rustc, cargo 등)의 무게를 줄임으로써 더욱 크기를 줄일 수 있었다. 
![최적화 후 이미지 사이즈](img/img_1.png)

[ignore 추가와 slim 사용](dockerfile_history/upgrade)

<br>

**step3**
<br>

여기에서 slim이 아니라 더 원시적인 OS를 사용하면 더욱 최적화가 가능하다.  

![OS 변경](img/img_2.png)

보면 96.6MB로 줄였다. 처음에 7.34GB 와 비교하면 굉장한 변화다.  

[윈시적인 OS 사용](dockerfile_history/os)



<br>

## 보안 제약 사항

우리가 대비해야 하는 것
* 도스 공격(denial-of-service attack, Dos attack): 서비스를 다운시켜 다른 사람들이 회원가입을 하지 못하게 한다. 모든 온라인 서비스의 공통적 위협
* 데이터 갈취(data theft): 거대한 이메일 주소 목록을 훔친다. 
* 피싱(phishing): 우리 서비스를 사용해서 합법적으로 보이는 이메일을 피해자에게 보내, 그들로 하여금 어떤 링크를 클릭하거나 다른 행동을 수행하게 속인다. 


## 이메일 검증

* email과 name에 대한 도메인 검증
* email 사용자의 확인
  * 인증 메일 보낸 후 확인

### 확인 이메일

* 이메일을 전송하는 모듈을 작성
* 기존의 POST /subscriptions 요청 핸들러 로직을 적용해서 새로운 명세를 매칭
* GET /subscriptions/confirm 요청 핸들러를 새로 작성

#### Arc

```rust
pub fn run(
  listener: TcpListener,
  db_pool: PgPool,
  email_client: EmailClient,
) -> Result<Server, std::io::Error> {
  let db_pool = Data::new(db_pool);
  // actix_web::web::data(Arc 포인터)로 EmailClient를 감싸고 App을 만들어야 할 때마다 포인터를 app_data로 보낸다. 
  // 아래 부분 없이도 동작은 가능하다. 그럼 어떤게 최선일까
  // EmailClient가 단순히 Client 인스턴스를 감싼 래퍼라면, 굳이 Arc를 사용해서 커넥션 풀을 두 번 감쌀 필요가 없다. 
  // 그러나 현재 EmailClient는 base_url과 sender 이렇게 두 개의 필드를 가지고 있다. Arc를 사용하지 않으면 App 인스턴스가 생성될 때마다 
  // 해당 데이터의 사본을 새로운 메모리에 할당한다. 한편 Arc로 감싼 경우 모든 인스턴스가 이걸 공유한다. 
  // 하지만 기억해야 할 건 스레드마다 App 인스턴스를 만든다는 것이다. 
  let email_client = Data::new(email_client); // 이부분을 주석하는 것이 Arc를 사용하지 않는것. 
  let server = HttpServer::new(move || {
    App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
  })
          .listen(listener)?
          .run();
  Ok(server)
}
```


* I/O 동작을 수행할 때는 항상 타임아웃을 설정하도록 하자. 


### 버그 

apply fix 가 이상하게 동작한다.
Debug가 없어 컴파일에러가 발생하는 상황에서 `#[derive(Debug)]`를 추가하라고 apply fix 가 떴지만 이상한 곳을 라우팅하고 있다.  
rust-lang 버그인지 tracing 버그인지는 잘모르겠는데 시간날 때 알아보자. 

![img](img/img_3.png)

`email_client`에 가서 변경해야 하는데 엉뚱한 데 use 하는데 가있다. 
![img](img/img_4.png)

### 에러 핸들링

오류는 두 가지 목적을 갖음
(제인 러스비가 발표함) 후회하지 않을 영상이니 꼭 시청하자. https://youtu.be/rAF8mLI0naQ?si=rZGD1ty0Wya1KUu0  

* 제어 흐름(다음으로 할 것을 결정)
* 보고(사실 이후에 무엇이 잘못되었는지 조사)

오류의 위치에 기반해 다음을 구분할 수 있다.
* 내부(애플리케이션 안에서 다른 함수를 부르는 함수)
* 경계(목적을 달성하는 데 실패한 API 요청)

|       | 내부          | 경계    |
|-------|-------------|-------|
| 제어 흐름 | 타입, 메서드, 필드 | 상태 코드 |
| 보고    | 로그,트레이스     | 응답 바디 |


에러 처리를 세부하게 하다보니 보일러 플레이트 코드가 굉장히 많아졌다.
이를 줄이기 위해 `thiserror`가 등장했다. `#[derive(thiserror::Error)]` 콘텍스트 안에서 다른 속성들에 접근해 우리가 원하는 동작을 수행할 수 있다.

* `#[error(/* */)]`: enum 변형인 Display 표현을 정의
* `#[source]` Error::source에서 근본 원인으로 반환할 것을 지정하기 위해 사용
* `#[from]`: 적용된 타입에 대한 From의 구현을 최상위 오류 타입으로 자동 파생


anyhow를 사용해서 오류 메시지 표현을 편리하게 하였다.
anyhow와 thiserror의 차이를 이해해보려고 하자. 


### 뉴스레터 전달

**구현 전략**

* 유입되는 API 호출 바디에서 뉴스레터 발행의 세부 정보를 꺼낸다.
* 데이터베이스에 확인된 모든 구독자 목록을 꺼낸다.
* 전체 리스트에 대해 다음을 반복한다. 
  * 구독자의 이메일을 얻는다.
  * Postmark를 통해 이메일은 전송한다. 


### 현재 접근 방식의 한계

* 보안
  * 엔드포인트는 보호되어 있지 않아 누구나 요청을 던질 수 있고, 모든 청중에게 확인하지 않고 정보를 뿌릴 수 있음
* 성능
  * 한 번에 한 통의 메일만 보내고 있음
* 내결함성
  * 하나의 이메일을 보내는데 실패하면 ?를 사용해서 오류를 부풀리고 500흘 호출자에게 반환. 나머지 이메일들은 절대로 전송되지 않으며 실패한 이메일도 재전송하지 않음
* 재시도 안정성 
  * 네트워크를 통한 통신에서는 많은 것이 잘못될 수 있음. API 소비자들이 서비스를 호출했을 때 타임아웃이나 500을 경험하면 어떻게 대처해야 하는가. 이들은 재시도할 수 없음. 


### API 보호

**인증**을 사용하여 확인된 사용자만 /newsletters API를 호출할 수 있도록 하자. 
그러면 어떻게 **인증**(authenticate)해야 할까. 인증 방법은 크게 세 가지가 있다. 

1. 사용자들의 지식: 비밀번호, PIN, 보안 질문
2. 사용자들의 소유물: 스마트폰, 인증 앱
3. 사용쟈들의 존재: 지문, face id

각 접근 방식에는 고유의 약점, 단점이 있다.

**사용자들의 지식**  
비밀번호가 짧은 경우 무차별 대입 공격에 취약하고, 또 그렇다고 길게 가져가면 사용자가 기억하기 힘들어 진다. 그렇다고 모든 사이트의 동일한
비밀번호를 사용하는 것도 굉장히 위험하다. 사용자들은 충분히 길고 고유한 비밀번호를 관리해야 하는데 이는 쉽지 않다.

**사용자들의 소유물**  
스마트폰이나 U2F 키는 분실 가능성이 있으며, 분실할 경우 사용자의 계정이 잠긴다. 소유물은 도난이나 오용될 수 있으며 이 경우 공격자에게는 피해자를 
모방할 수 있는 기회가 생긴다.

**사용자들의 존재**  
생물학적 특성은 비밀번호와 달리 변경할 수 없다. 그런데 지문을 위조하는 것은 대부분의 사람들이 생각하는 것보다 쉽다. 생물학적 특성은 그것을 
남용하거나 잃어버릴지도 모르는 정부 기관들이 종종 이용할 수 있는 정보다.

<br>
각 접근 방식이 고유한 단점을 가지고 있기 때문에 다요소 인증(multi-factor authentication, MFA)을 사용한다. 이는 두 가지 이상의 다른 유형의 인증 요소를 
제공해야만 대상에 접근할 수 있게 한다. 

<br>

#### 비밀번호 저장소

데이터베이스에 비밀번호를 저장하려면 암호화 해시 함수가 필요하다. 해시 함수는 입력 공간의 문자열을 고정된 길이의 출력으로 매핑한다.
또한 입력에서 조금의 차이만 있어도 출력은 전혀 관계없을 정도로 변한다(눈사태 효과). 해시는 결정적 함수이다. 즉 입력이 같으면 출력도 같다. 
주의할 점은 해시 함수는 주입 가능하지 않는다는 점이다. 입력 공간이 유한하다고 가정하면 이론적으로 환벽한 해시 함수를 찾을 수 있다. 즉 f(x) == f(y) 이면 x == y가 된다. 
그러나 약간의 충돌할 여지가 있다. 


역상 공격, 사전 공격에 대한 용어 정리를 하고 넘어가자.  

공격자들이 users 테이블을 손에 넣었을 때 SHA3-256으로 사용자들의 비밀번호를 충분히 보호할 수 있나? 
공격자가 데이터베이스의 특정한 비밀번호 해시를 깨려고 한다고 가정해보자. 공격자는 심지어 원래 비밀번호를 꺼낼 필요도 없다. 성공적으로 인증하기 위해 그들은 깨고자 시도하는 
비밀번호와 SHA3-256이 매치하는 입력 문자열 s, 다시 말해 충돌을 찾기만 하면 된다. 이를 역상 공격(preimage attack)이라고 한다. 
브루트포스로 접근하면 지수적인 복잡성 2^n 이 걸린다. 이때 n은 해시의 길이(비트 단위)다. n > 128 이면 계산하기 불가능한 것으로 간주된다. 

원래 비밀번호에 대한 특정한 가정을 함으로써 검색 공간을 줄이며 단순한 사전 공격을 시도해볼 수 있다. 길이는 얼마나 되나, 어떤 기호들을 사용했는가?  
17 문자 미만의 알파벳과 숫자로 이루어진 비밀번호를 찾는다고 가정해보자.  

비밀번호 후보수는 아래와 같다.
```text
// 모든 허가된 비밀번호 길이에 대해 (26 소문자 + 10 개 숫자) ^ 비밀번호 길이
36 ^ 1 + 
36 ^ 2 + 
36 ^ 3 + 
36 ^ 4 + 
... 
36 ^ 16 + 
```

대략 8 x 10^24 개의 비밀번호를 사용할 수 있다. GPU를 사용해서 초당 ~9억 개의 SHA3-512(SHA3-256자료는 찾지 못함) 해시를 계산하였다. 
초당 ~10^9개의 해시를 계산한다고 가정했을 때, 모든 비밀번호 후보를 해시하는 데는 ~10^15 초가 걸린다. 대략적인 우주의 나이는 4x10^17 초다.
100만 개의 GPU를 사용해서 검색을 병렬적으로 처리하더라도 ~10^9초, 즉 거의 30년이라는 시간이 소요된다.  

하지만 지금 이야기는 비밀번호가 고유했을 때의 이야기다. 개인이 수백 개의 온라인 서비스에 설정한 고유한 비밀번호를 기억하기란 불가능하기에 대부분의 사용자들은
비밀번호 관리자에 의존하거나 여러 계정에서 하나 이상의 비밀번호를 재사용한다. 심지어 재사용하더라도 대부분의 비밀번호는 무작위와는 거리가 멀다. 일반적인 단어, 전체 이름, 
특정한 날짜, 유명한 스포츠 팀의 이름등을 사용한다. 공격자는 손쉽게 수천 개의 그럴듯한 비밀번호를 생성할 수 있는 간단한 알고리즘을 설계할 수 있다. 하지만 그럴 필요도 없이
지난 십년 동안 발생한 수많은 보안 침해 중 암호 데이터 세트를 확인해 가장 일반적인 비밀번호를 찾아 낼 수도 있다. 
공격자들은 몇 분도 안돼서 가장 많이 사용된 1억 개의 비밀번호에 대한 SHA3-256 해시를 미리 계산할 수 있다. 그러고 나서 데이터베이스를 스캐닝하면서
일치하는 것을 찾기 시작한다. 이를 사전 공격(dictionary attack)이라고 하며 매우 효과적인 공격이다. 


<br>

사용자들은 고유한 비밀번호를 만들고 기억하기 힘드니까 사용자는 고유한 비밀번호를 만들되, 우리 사이트에서 저장되는 비밀번호는 다르게 저장하자! 에서 나온 것이 소금 뿌리기이다. 
공격자가 데이터베이스의 모든 사용자에 관해 전체 사전을 재해시해야 한다면 훨씬 어려운 작업이 될 것이다. 소금 뿌리기는 각 사용자에 대해 고유한 무작위 문자열을 생성한다. 
이 소금은 사용자 비밀번호 앞에 추가되며 그 뒤에 해시가 생성된다. `PasswordHasher::hash_password`는 소금을 비밀번호 앞에 붙이는 작업을 처리한다.  
그리고 이 소금은 데이터베이스에서 비밀번호 해시 옆에 저장된다. 만약 공격자가 데이터베이스 백업을 손에 넣는다면, 모든 소금에 접근할 수 있다. (이것을 대비하기 위해 후추 뿌리기도 있다. 이는 데이터베이스에 저장된 모든 
해시는 애플리케이션만 알 수 있는 공유된 시크릿을 사용해서 암호화된다. ) 하지만 공격자들은 dictionary_size가 아닌 dictionary_size * n_users의 해시를 계산해야 한다. 또한 해시를 미리 계산하는 것도
선택할 수 없다. 결과적으로 우리는 보안 구멍을 발견하고 이에 대응할 시간을 벌 수 있다.


```shell
error[E0631]: type mismatch in function arguments
   --> src\routes\newsletters.rs:157:18
    |
116 |     UnexpectedError(#[from] anyhow::Error),
    |     --------------- found signature defined here
...
157 |         .map_err(PublishError::UnexpectedError)?;
    |          ------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected due to this
    |          |
    |          required by a bound introduced by this call
    |
    = note: expected function signature `fn(argon2::password_hash::Error) -> _`
               found function signature `fn(anyhow::Error) -> _`
note: required by a bound in `Result::<T, E>::map_err`
   --> C:\Users\User\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\result.rs:853:26
    |
853 |     pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F> {
    |                          ^^^^^^^^^^^^^^ required by this bound in `Result::<T, E>::map_err`
help: consider wrapping the function in a closure
    |
157 |         .map_err(|arg0: argon2::password_hash::Error| PublishError::UnexpectedError(/* anyhow::Error */))?;
    |                  ++++++++++++++++++++++++++++++++++++                              +++++++++++++++++++++

For more information about this error, try `rustc --explain E0631`.
```

<br>

non_existing_user_is_rejected()와 invalid_password_is_rejected() 테스트의 수행 속도가 차이난다. 이는 부채널 공격(side channel attack) 중에 하나인 소요 시간 분석의 여지를 준다.
즉 공격자가 최소한 유효한 사용자 이름을 알고 있다면, 이들은 서버의 응답 시간을 확인해서 다른 사용자 이름의 존재 여부를 확인할 수 있다. 이것은 잠재적인 사용자 열거형 취약점(user enumeration vulnerability)에 해당한다. 

어떻게 해결해야 할까  

1. 유효하지 않은 비밀번호에 의한 인증 실패 시간과 존재하지 않는 사용자 이름에 의한 인증 실패 시간 차이를 제거 한다.
2. 특정 IP/사용자 이름으로부터의 인증 시도 실패 횟수를 제한한다. 

1번 방법에 대해 더 살펴보자. 
시간 차이를 제거하기 위해서는 두 가지 경우에 동일한 양의 작업을 수행해야 한다. 
1. 주어진 username에 대해 저장한 크리덴셜을 꺼낸다.
2. 크리덴셜이 존재하지 않으면 401을 반환한다.
3. 크리덴셜이 존재하면 비밀번호 후보를 해싱한 뒤 저장한 해시와 비교한다. 

조기 이탈은 제거해야 한다. 암호 후보의 해시와 비교할 수 있는 폴백 에상 암호(소금과 부하 파라미터를 포함한)를 가져야 한다. 

코드를 수정하여 정적으로 확연한 타이밍이 발생하지 않도록 하였다.


### xss

사이트 간 스크립팅(cross-site scripting) 공격은 공격자가 신뢰할 수 없는 소스(사용자 입력, 쿼리 파라미터 등)로부터 만들어진 동적 콘텐츠를 악용함으로써
신뢰할 수 있는 웹 사이트에 HTML 조각이나 자바스크립트 스니펫을 주입한다. OWASP는 XSS 공격을 방지하는 방법에 관한 전체적인 치트 시트(https://cheatsheetseries.owasp.org/cheatsheets/Cross_Site_Scripting_Prevention_Cheat_Sheet.html)를 제공한다.
OWASP 가이드라인에 따르면 신뢰할 수 없는 입력을 HTML 엔티티로 인코딩해야 한다.

* `&` -> `&amp;`
* `<` -> `&lt;`
* `>` -> `&gt;`
* `"` -> `&quot;`
* `'` -> `&#x27;`
* `/` -> `&#x2F;`


### 도서 글귀

* 영속성 요규가 명확하지 않다면, 관게형 데이터베이스를 사용하자. 큰 확장을 예상할 필요가 없다면, PostgreSQL을 사용하자.
* 스레드는 병렬로 동작한다. 비동기는 병렬로 대기한다. 
* 관측 가능성이란 여러분의 환경에 관해 임의로 질문을 던질 수 있는 속성을 말한다. 여기서 가장 중요한 것은 여러분이 무엇을 질문하기 원하는지 미리 알 필요가 없다는 점이다. 

### 학습 키워드 

* Arc: connection을 Arc에 담아서 넘겼다. 
* TryInfo: .try_into()와 try_from()을 적재적소에 사용하자.
* [AsyncDrop을 지원하지 않는 이유](https://github.com/rust-lang/rfcs/pull/2958)
* 트랜잭션 begin 하고 commit, rollback 둘 중 하나라도 하지 않으면 에러 발생.


### TODO LIST

* 사용자가 구독을 두 번 한다면? 두 번의 확인 이메일을 받는 것을 보장하자.
* 사용자가 확인 링크를 두 번 클릭하면 어떻게 되나
* 구독 토큰의 형태는 적절하지만 토큰이 실제로는 존재하지 않는다면,
* 유입되는 토큰에 대해 검증하기, 현재 사용자의 입력을 그대로 쿼리에 전달하고 있다. (다행이도 sqlx에서 sql 인젝션을 방지해준다.)
* 관리자 대시보드에 send a newsletter issue 링크를 추가한다.
* GET /admin/newsletters에 새로운 이슈를 제출하는 HTML 폼을 추가한다.
* POST /newsletter가 그 폼 데이터를 처리하게 수정한다.
  * 경로를 POST /admin/newsletter로 변경한다.
  * 기본 인증을 세션 기반 인증으로 마이그레이션한다.
  * Json 추출기(application/json) 대신 Form 추출기(application/x-www-form-urlencoded)를 사용해서 요청 바디를 처리한다.
  * 테스트 스위트를 수정한다.

[깃허브 저장소](https://github.com/LukeMathWalker/zero-to-production)
