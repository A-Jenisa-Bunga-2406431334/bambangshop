use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::to_string;
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    pub fn update(&self, payload: Notification) {
        let url = self.url.clone();
        let payload_clone = payload.clone();
        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let _ = client.post(&url)
                .header("Content-Type", "application/json")
                .body(to_string(&payload_clone).unwrap())
                .send();
            rocket::warn!("Sent {} notification of: [{}] {}, to: {}",
                payload_clone.status, payload_clone.product_type,
                payload_clone.product_title, url);
        });
    }
}