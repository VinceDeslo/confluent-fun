use crate::user::service::Service;
use rdkafka::{
    producer::BaseRecord,
};

pub fn delete_user_event(
    service: &Service,
    id: i32
) {
    let topic = "delete_user";
    
    let data = serde_json::to_string(&id)
        .expect("Failed to serialize delete user event payload.");
    
    let key = &format!("key-{}", id);
    let payload = &format!("payload-{}", data);

    let record = BaseRecord::to(topic)
        .key(key)
        .payload(payload);

    service.event_producer.producer
        .send(record)
        .expect("Failed to send delete user event");
}