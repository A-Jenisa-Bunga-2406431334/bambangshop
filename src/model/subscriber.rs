use rocket::serde::{Serialize, Deserialize};
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    pub fn update(&self, product_type: &str, notification: &Notification) {
        let client = reqwest::blocking::Client::new();
        let _ = client.post(format!("{}/receive", self.url))
            .json(notification)
            .send();
    }
}