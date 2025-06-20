use redis::{Commands, FromRedisValue, RedisError, RedisResult};
use redis::JsonCommands;
use serde::{Deserialize, Serialize};
use serde_json;


#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    uid: u32,
    name: String,
    tier: u8,
    match_num: u32,
}

impl FromRedisValue for UserInfo {
    fn from_redis_value(value: &redis::Value) -> RedisResult<Self> {
        if let redis::Value::Array(data) = value {
            let mut user_info = UserInfo {
                uid: 0,
                name: String::new(),
                tier: 0,
                match_num: 0,
            };

            let iter = data.chunks_exact(2);
            for chunk in iter {
                if let [key, val] = chunk {
                    let key_str: String = redis::from_redis_value(key)?;
                    match key_str.as_str() {
                        "uid" => user_info.uid = redis::from_redis_value(val)?,
                        "name" => user_info.name = redis::from_redis_value(val)?,
                        "tier" => user_info.tier = redis::from_redis_value(val)?,
                        "match_num" => user_info.match_num = redis::from_redis_value(val)?,
                        _ => {}
                    }
                }
                else {
                    return Err(RedisError::from((
                        redis::ErrorKind::TypeError, 
                        "Expected key-value pairs in array"
                    )));
                }
            }

            Ok(user_info)
        }
        else {
            Err(RedisError::from((
                redis::ErrorKind::TypeError, 
                "Cannot convert to UserInfo"
            )))
        }
    }
}


fn main() -> RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut connection = client.get_connection()?;

    {
        let _: () = connection.hset("user:1", "uid", 1)?;
        let _: () = connection.hset("user:1", "name", "Steve")?;
        let _: () = connection.hset("user:1", "tier", 0)?;
        let _: () = connection.hset("user:1", "match_num", 8)?;

        let _: () = connection.hset("user:2", "uid", 3125)?;
        let _: () = connection.hset("user:2", "name", "초코송이")?;
        let _: () = connection.hset("user:2", "tier", 3)?;
        let _: () = connection.hset("user:2", "match_num", 73)?;

        let user1: UserInfo = connection.hgetall("user:1")?;
        let user2: UserInfo = connection.hgetall("user:2")?;

        println!("User 1: {:?}", user1);
        println!("User 2: {:?}", user2);
    }

    // {
    //     let user3 = UserInfo {
    //         uid: 3,
    //         name: "Alice".to_string(),
    //         tier: 1,
    //         match_num: 10,
    //     };
    //     let user4 = UserInfo {
    //         uid: 4,
    //         name: "김가루".to_string(),
    //         tier: 2,
    //         match_num: 20,
    //     };
    //     let user3_json = serde_json::to_string(&user3)?;
    //     let user4_json = serde_json::to_string(&user4)?;

    //     let _:() = connection.json_set("user:3", "$", &user3_json)?;
    //     let _:() = connection.json_set("user:4", "$", &user4_json)?;

    //     let user3: UserInfo = connection.json_get("user:3", "$")?;
    //     let user4: UserInfo = connection.json_get("user:4", "$")?;

    //     println!("User 3: {:?}", user3);
    //     println!("User 4: {:?}", user4);

    //     let user3_uid: u32 = connection.json_get("user:3", "$.uid")?;
    //     let user4_name: String = connection.json_get("user:4", "$.name")?;
        
    //     println!("User 3 UID: {}", user3_uid);
    //     println!("User 4 Name: {}", user4_name);
    // }

    Ok(())
}