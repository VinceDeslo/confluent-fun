use crate::user::service::Service;
use rdkafka::{
    producer::BaseRecord,
};
use crate::models::User;

pub fn update_user_event(
    service: &Service,
    user: &User
) {
    let topic = "update_user";
    
    let data = serde_json::to_string(&user)
        .expect("Failed to serialize update user event payload.");
    
    let key = &format!("key-{}", user.id);
    let payload = &format!("payload-{}", data);

    let record = BaseRecord::to(topic)
        .key(key)
        .payload(payload);

    service.event_producer.producer
        .send(record)
        .expect("Failed to send update user event");
}