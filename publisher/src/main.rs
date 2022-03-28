use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use paho_mqtt::AsyncClient;

#[tokio::main]
async fn main() {
    let broker_url =
        env::var("MQTT_BROKER_URL").unwrap_or_else(|_| "tcp://localhost:1883".to_string());
    let broker_topic = env::var("MQTT_BROKER_TOPIC").unwrap_or_else(|_| "testTopic".to_string());
    let client = AsyncClient::new(broker_url).expect("Error creating the client");
    println!("Connecting to the MQTT server");
    let token = client.connect(None).await;
    if let Err(err) = token {
        println!("Error: {}", err);
    }

    let mut lines = Vec::<String>::with_capacity(1000);
    for line in BufReader::new(File::open("banner.json").expect("Error opening file")).lines() {
        lines.push(line.unwrap());
    }
    println!("Publishing messages on the topic `{}`", broker_topic);
    let mut cond = true;
    while cond {
        for line in &lines {
            let msg = paho_mqtt::Message::new(&broker_topic, line.as_bytes(), paho_mqtt::QOS_1);
            if let Err(err) = client.publish(msg).await {
                println!("Publishing error: {}", err);
                cond = false;
                break;
            }
        }
    }

    // Disconnect from the broker
    println!("Disconnecting");
    if let Err(err) = client.disconnect(None).await {
        println!("Disconnecting error: {}", err);
    }
}
