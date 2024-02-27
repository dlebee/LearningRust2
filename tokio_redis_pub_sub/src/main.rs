use std::str::from_utf8;
use std::time::Duration;
use tokio_stream::{StreamExt};
use tokio::time::sleep;
use mini_redis::client;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber
        .into_stream()

        .map(|msg| -> Option<i32> {
            match msg {
                Ok(msg) => {
                    match from_utf8(&msg.content) {
                        Ok(msg_content_str) => {
                            match String::from(msg_content_str).parse::<i32>() {
                                Ok(number) => Some(number),
                                _ => None
                            }
                        },
                        _ => None
                    }
                },
                _ => None
            }
        })
        .filter(|msg| -> bool {
            match msg {
                Some(_) => true,
                None => false
            }
        })
        .map(|msg| -> i32 {
            msg.unwrap()
        });

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {




    tokio::spawn(async {
        publish().await;
    });

    subscribe().await?;
    println!("DONE");

    Ok(())
}
