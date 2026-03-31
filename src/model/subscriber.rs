use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
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
        let payload = to_string(&notification).unwrap();
        rocket::info!("Sending notification to {} with payload: {}", url, payload);
        let request = REQWEST_CLIENT.post(&url).json(&notification);
        tokio::spawn(async move {
            let response = request.send().await;
            match response {
                Ok(res) => rocket::info!("Notification sent to {} with status: {}", url, res.status()),
                Err(e) => rocket::error!("Failed to send notification to {}: {}", url, e),
            }
        });
    }
}