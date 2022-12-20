// # Required connection configs for Kafka producer, consumer, and admin
// bootstrap.servers=pkc-ld537.ca-central-1.aws.confluent.cloud:9092
// security.protocol=SASL_SSL
// sasl.mechanisms=PLAIN
// sasl.username={{ CONFLUENT_API_KEY}}
// sasl.password={{ CONFLUENT_API_SECRET}}

// # Best practice for higher availability in librdkafka clients prior to 1.7
// session.timeout.ms=45000

use crate::config::Config;
use rdkafka::{
    ClientConfig,
    producer::BaseProducer
};

pub struct EventProducer {
    pub producer: BaseProducer
}

impl EventProducer {
    pub fn new(config: &Config) -> Self {
        Self {
            producer: ClientConfig::new()
                .set("bootstrap.servers", &config.confluent_bootstrap_server)
                .set("security.protocol", "SASL_SSL")
                .set("sasl.mechanisms", "PLAIN")
                .set("sasl.username", &config.confluent_api_key)
                .set("sasl.password", &config.confluent_api_secret)
                .create()
                .expect("invalid producer config"),
        }
    }
}