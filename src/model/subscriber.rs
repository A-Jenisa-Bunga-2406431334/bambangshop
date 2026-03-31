use rocket::serde::{Serialize, Deserialize};
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    pub fn update(&self, _product_type: &str, notification: &Notification) {
        let url = format!("{}/receive", self.url);
        let notification = notification.clone();
        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let _ = client.post(&url)
                .json(&notification)
                .send();
        });
    }
}