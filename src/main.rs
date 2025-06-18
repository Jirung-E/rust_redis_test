use redis::Commands;


fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut connection = client.get_connection()?;

    let _: () = connection.set("test_value", 100)?;
    let value: i32 = connection.get("test_value")?;
    
    println!("Got value: {}", value);

    let _: () = connection.incr("test_value", 5)?;
    let value: i32 = connection.get("test_value")?;

    println!("Got value: {}", value);

    let _: () = connection.decr("test_value", 3)?;
    let value: i32 = connection.get("test_value")?;

    println!("Got value: {}", value);

    Ok(())
}