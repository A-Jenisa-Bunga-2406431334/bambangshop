use rocket::http::Status;
use bambangshop::{Result, compose_error_response};
use crate::model::notification::Notification;
use crate::model::product::Product;
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

    pub fn notify(product_type: &str, notification: Notification) {
        let subscribers = SubscriberRepository::list_all(product_type);
        for subscriber in subscribers {
            subscriber.update(product_type, &notification);
        }
    }

    pub fn publish(product: &Product, subscriber_name: String) -> Result<Notification> {
        let notification = Notification {
            product_type: product.product_type.clone(),
            product_title: product.title.clone(),
            product_url: product.get_url(),
            subscriber_name,
            status: String::from("PROMOTED"),
        };
        NotificationService::notify(&product.product_type, notification.clone());
        return Ok(notification);
    }
}