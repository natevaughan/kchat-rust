use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::ClientConfig;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Initializing kchat...");
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "10.201.254.146:9092")
        .create()
        .expect("invalid producer config");
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        stdin.read_line(&mut buffer).expect("nothing came in");
        let now = SystemTime::now();
        producer
            .send(
                BaseRecord::to("kchat-messages")
                    .key(&format!(
                        "kchat-{:?}",
                        now.duration_since(UNIX_EPOCH).expect("Time went backwards")
                    ))
                    .payload(&buffer),
            )
            .expect("Failed to send message");
    }
}
