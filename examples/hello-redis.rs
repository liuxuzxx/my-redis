use mini_redis::client;
use mini_redis::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = String::from("127.0.0.1:7379");
    let mut client = client::connect(addr).await?;

    let key = String::from("hello");

    client.set(&key, "world".into()).await?;
    let result = client.get(&key).await?;
    println!("get value from redis server; result:{:?}", result);
    Ok(())
}
