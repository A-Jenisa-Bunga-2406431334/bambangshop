use rocket::http::Status;
use bambangshop::{Result, compose_error_response};
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let subscriber_result = SubscriberRepository::add(product_type, subscriber);
        return Ok(subscriber_result);
    }

    pub fn unsubscribe(product_type: &str, url: &str) -> Result<Subscriber> {
        let subscriber_opt = SubscriberRepository::delete(product_type, url);
        if subscriber_opt.is_none() {
            return Err(compose_error_response(
                Status::NotFound,
                String::from("Subscriber not found.")
            ));
        }
        return Ok(subscriber_opt.unwrap());
    }
}