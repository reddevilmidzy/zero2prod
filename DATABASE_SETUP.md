# 데이터베이스 셋업

책에 너무 빠르게 쇽쇽 지나가서 마주했던 오류 남겨보자 한다.  

우선 진행했던 사항은 아래와 같다.

1. 도커 설치
2. sqlx-cli 설치
3. psql 설치

### 도커 설치
이건 그냥 냅다 구글에 치는게 빠르다.

### sqlx-cli 설치

```shell
cargo install --version="~0.6" sqlx-cli --no-default-features --features restls,postgres
```

러스트의 패키지 매니저인 cargo 명령으로 쉽게 설치가 가능하다. 

### psql 설치
여기서 살짝 애먹었는데, [링크](https://www.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows)에서 자신의 운영체제에 
맞는 녀석으로 설치하면된다. 그리고 윈도우라면 시스템 환경 변수 설정을 해주도록 하자. Postgres를 설치해서 하는 것이 아니기 때문에 자동으로 환경 변수 등록이 되지 않는다.  

