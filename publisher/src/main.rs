use std::env;

use paho_mqtt::AsyncClient;
use tokio::time;

#[tokio::main]
async fn main() {
    let broker_url =
        env::var("MQTT_BROKER_URL").unwrap_or_else(|_| "tcp://localhost:1883".to_string());
    let client = AsyncClient::new(broker_url).expect("Error creating the client");
    println!("Connecting to the MQTT server");
    let token = client.connect(None).await;
    if let Err(err) = token {
        println!("Error: {}", err);
    }

    let mut count = 0;
    loop {
        println!("Publishing a message on the topic 'testTopic'");
        let msg = paho_mqtt::Message::new("testTopic", "Hello Rust MQTT world!", paho_mqtt::QOS_1);
        if let Err(err) = client.publish(msg).await {
            println!("Publishing error: {}", err);
        }
        time::sleep(time::Duration::from_secs(1)).await;
        if count == 100 {
            break;
        }
        count += 1;
    }

    // Disconnect from the broker
    println!("Disconnecting");
    if let Err(err) = client.disconnect(None).await {
        println!("Disconnecting error: {}", err);
    }
}
