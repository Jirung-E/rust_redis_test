# Redis

## 명령
### 기본
- SET/GET/MGET
    > SET: 단일 키에 값 저장  
    > GET: 단일 키의 값 조회  
    > MGET: 여러 키의 값 조회  
- INCR/DECR
    > 키의 값을 증감  
- DEL
    > 키 삭제

### 자료구조
#### Hash
- HSET/HGET
    > HSET: 해시 키에 필드와 값을 저장  
    > HGET: 해시 키의 특정 필드 값 조회
- HGETALL
    > 해시 키의 모든 필드와 값 조회

다른 자료구조는 일단 생략

### Json
- JSON.SET/JSON.GET
- RedisJSON 모듈을 별도로 설치해야하므로 생략

- 그냥 JSON 문자열로 저장하고 불러와서 파싱하는 방법도 있다.


## 저장
- SAVE/BGSAVE
    > 동기식/비동기식

따로 저장하지 않아도 일정 시간마다 자동으로 백업/로드(컴퓨터를 껐다 켜도 유지됨)

redis-cli로 접속해서 다음 명령을 입력하면 저장된 시점을 확인할 수 있다.
```bash
LASTSAVE
```

- 한 게임이 종료될때 게임 결과를 저장해야한다.
- 게임 결과는 좀 누락돼도 큰 문제는 없다.

- RDB/AOF
    > RDB: 주기적으로 스냅샷을 저장하는 방식. 다음 저장 시점 이전(또는 저장 도중)에 장애가 발생하면 데이터 손실이 있을 수 있음.  
    > AOF: 모든 쓰기 명령을 로그로 기록하는 방식. 용량이 크지만 데이터 손실이 거의 없음.  
    > redis.conf 파일에서 설정 가능

- 게임이 종료될때마다 결과를 저장해야하는데 그때마다 전체 DB를 저장하는것은 비효율적이므로 RDB/AOF를 적절히 섞어서 사용



- - -



모든 플레이어 데이터를 메모리에 로드해두는건 비효율적이므로 접속중인 플레이어 정보만 로드하는게 좋지만, 그런 기능은 없는것같다.  
(그렇게 하려면 파일(또는 관계형 데이터베이스 등)로 따로 저장한 다음 서버 시작시 redis 초기화 후 필요한 정보만 로드해야함)  

지금은 용량이 크지 않아서 큰 문제는 없을것같다.  
(100판 플레이한 플레이어의 데이터 크기를 대략 4KB로 가정하면 플레이어 1000명의 데이터 크기는 약 4MB)


```rs
struct UserInfo {
    id: u32,
    name: String,
    tier: u32,
    match_results: Vec<MatchResult>,
}

struct MatchResult {
    kills: u32,
    deaths: u32,
    damage_done: u32,
    win: bool,
}
```

```json
{
    "id": 1,
    "name": "Player1",
    "tier": 2,
    "match_results": [
        {
            "kills": 10,
            "deaths": 2,
            "damage_done": 3000,
            "win": true
        },
        {
            "kills": 5,
            "deaths": 3,
            "damage_done": 1500,
            "win": false
        }
    ]
}
```
