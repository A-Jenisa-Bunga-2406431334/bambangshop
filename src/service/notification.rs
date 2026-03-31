use rocket::http::Status;
use bambangshop::{Result, compose_error_response};
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        todo!()
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        todo!()
    }
}